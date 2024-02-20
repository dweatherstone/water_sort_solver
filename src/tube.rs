use std::fmt::Display;

use crate::{game::Move, TUBE_SIZE};

#[derive(Clone)]
pub struct ColourPos {
    pub colour: String,
    pub pos: usize,
    pub block_size: usize,
}

#[derive(Clone)]
pub struct Tube {
    pub contents: Vec<Option<String>>,
    pub tube_number: usize,
}

impl Tube {
    pub fn from_string(string_colours: String, tube_number: usize) -> Tube {
        let mut colours = Vec::with_capacity(TUBE_SIZE);
        let vec_string_colours: Vec<String> = string_colours
            .split(',')
            .map(|x| x.trim().to_lowercase())
            .collect();
        // Add empty cells where there is no string colour supplied
        for _ in vec_string_colours.len()..TUBE_SIZE {
            colours.push(None);
        }
        // Add the colours from the string. Note that any unmatched strings will get set as Empty.
        for str_col in vec_string_colours {
            if &str_col == "empty" || str_col.is_empty() {
                colours.push(None);
            } else {
                colours.push(Some(str_col));
            }
        }

        Tube {
            contents: colours,
            tube_number,
        }
    }

    pub fn from_string_vec(colours: Vec<Option<String>>, tube_number: usize) -> Tube {
        let mut contents = Vec::new();
        for colour in colours.iter() {
            match colour {
                None => contents.push(None),
                Some(colour) => {
                    let colour = colour.trim().to_lowercase();
                    if colour == *"empty" {
                        contents.push(None);
                    } else {
                        contents.push(Some(colour));
                    }
                }
            }
        }
        Tube {
            contents,
            tube_number,
        }
    }

    pub fn is_valid_move_from(&self, a_move: &Move) -> bool {
        if self.tube_number != a_move.tube_from {
            return false;
        }
        let start = match self.get_top_colour() {
            Some(col_pos) => col_pos.pos,
            None => 0,
        };
        if start + a_move.quantity > TUBE_SIZE {
            return false;
        }
        for idx in start..start + a_move.quantity {
            match &self.contents[idx] {
                Some(col) => {
                    if col != &a_move.colour {
                        return false;
                    }
                }
                None => return false,
            }
        }
        true
    }

    pub fn is_valid_move_to(&self, a_move: &Move) -> bool {
        if self.tube_number != a_move.tube_to {
            return false;
        }
        let (top_colour, start) = match self.get_top_colour() {
            Some(top_col) => (top_col.colour, top_col.pos),
            None => (a_move.colour.clone(), TUBE_SIZE),
        };
        if (start as i32 - a_move.quantity as i32) < 0 {
            return false;
        }
        if top_colour != a_move.colour {
            return false;
        }

        true
    }

    pub fn pour_from(&mut self, a_move: &Move) {
        let mut qty = a_move.quantity;
        let col = &a_move.colour;
        for cell in self.contents.iter_mut() {
            if qty == 0 {
                break;
            }
            match cell {
                Some(c) => {
                    if c == col {
                        *cell = None;
                        qty -= 1;
                    } else {
                        break;
                    }
                }
                None => continue,
            }
        }
    }

    pub fn pour_to(&mut self, a_move: &Move) {
        let top_col = self.get_top_colour();
        let start = match top_col {
            Some(ref the_top) => the_top.pos - a_move.quantity,
            None => TUBE_SIZE - a_move.quantity,
        };
        let end = match top_col {
            Some(the_top) => the_top.pos,
            None => TUBE_SIZE,
        };
        for idx in start..end {
            self.contents[idx] = Some(a_move.colour.clone());
        }
    }

    pub fn get_top_colour(&self) -> Option<ColourPos> {
        for (pos, colour) in self.contents.iter().enumerate() {
            match colour {
                Some(col) => {
                    return Some(ColourPos {
                        colour: col.to_string(),
                        pos,
                        block_size: self.get_block_size(pos, col),
                    })
                }
                None => {}
            }
        }
        None
    }

    pub fn is_tube_all_same_contents(&self) -> bool {
        let mut iter = self.contents.iter();
        let first = iter.next();
        iter.fold(first, |acc, item| {
            acc.and_then(|stored| if stored == item { Some(stored) } else { None })
        })
        .is_some()
    }

    fn get_block_size(&self, start: usize, colour: &String) -> usize {
        let mut block_size = 0;
        for idx in start..self.contents.len() {
            match &self.contents[idx] {
                Some(col) => {
                    if col == colour {
                        block_size += 1;
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }

        block_size
    }
}

impl Display for Tube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        let mut colours = Vec::new();

        for colour in self.contents.iter() {
            let col = match colour {
                Some(c) => String::from(c),
                None => String::from("empty"),
            };
            colours.push(col);
        }

        out.push_str(format!("{}: (", self.tube_number + 1).as_str());
        out.push_str(colours.join(", ").as_str());
        out.push(')');

        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_setup() {
        let string_tests = vec![
            (
                String::from("red, red, blue, green"),
                Tube {
                    contents: vec![
                        Some("red".to_string()),
                        Some("red".to_string()),
                        Some("blue".to_string()),
                        Some("green".to_string()),
                    ],
                    tube_number: 1,
                },
            ),
            (
                String::from("empty, red, blue, green"),
                Tube {
                    contents: vec![
                        None,
                        Some("red".to_string()),
                        Some("blue".to_string()),
                        Some("green".to_string()),
                    ],
                    tube_number: 2,
                },
            ),
            (
                String::from("red, blue, green"),
                Tube {
                    contents: vec![
                        None,
                        Some("red".to_string()),
                        Some("blue".to_string()),
                        Some("green".to_string()),
                    ],
                    tube_number: 3,
                },
            ),
            (
                String::from("blue, green"),
                Tube {
                    contents: vec![
                        None,
                        None,
                        Some("blue".to_string()),
                        Some("green".to_string()),
                    ],
                    tube_number: 4,
                },
            ),
            (
                String::from("RED, rEd, Blue    ,    Green      "),
                Tube {
                    contents: vec![
                        Some("red".to_string()),
                        Some("red".to_string()),
                        Some("blue".to_string()),
                        Some("green".to_string()),
                    ],
                    tube_number: 5,
                },
            ),
            (
                String::from(""),
                Tube {
                    contents: vec![None; 4],
                    tube_number: 6,
                },
            ),
            (
                String::from("         ,      ,   ,"),
                Tube {
                    contents: vec![None; 4],
                    tube_number: 7,
                },
            ),
        ];

        for (idx, test) in string_tests.into_iter().enumerate() {
            let result = Tube::from_string(test.0, idx + 1);
            test_tube(&result, &test.1);
        }
    }

    #[test]
    fn test_colour_vec_setup() {
        let tests = vec![
            (
                vec![
                    Some("red".to_string()),
                    Some("green".to_string()),
                    Some("blue".to_string()),
                    Some("purple".to_string()),
                ],
                Tube {
                    contents: vec![
                        Some("red".to_string()),
                        Some("green".to_string()),
                        Some("blue".to_string()),
                        Some("purple".to_string()),
                    ],
                    tube_number: 1,
                },
            ),
            (
                vec![
                    None,
                    None,
                    Some("blue".to_string()),
                    Some("purple".to_string()),
                ],
                Tube {
                    contents: vec![
                        None,
                        None,
                        Some("blue".to_string()),
                        Some("purple".to_string()),
                    ],
                    tube_number: 2,
                },
            ),
            (
                vec![None, None, None, None],
                Tube {
                    contents: vec![None, None, None, None],
                    tube_number: 3,
                },
            ),
        ];
        for (idx, test) in tests.into_iter().enumerate() {
            let result = Tube::from_string_vec(test.0, idx + 1);
            test_tube(&result, &test.1);
        }
    }

    #[test]
    fn test_top_colour() {
        let tests = vec![
            (
                Tube::from_string(String::from("red, red, blue, green"), 1),
                Some(ColourPos {
                    colour: "red".to_string(),
                    pos: 0,
                    block_size: 2,
                }),
            ),
            (
                Tube::from_string(String::from("empty, red, blue, green"), 2),
                Some(ColourPos {
                    colour: "red".to_string(),
                    pos: 1,
                    block_size: 1,
                }),
            ),
            (
                Tube::from_string(String::from("red, blue, green"), 3),
                Some(ColourPos {
                    colour: "red".to_string(),
                    pos: 1,
                    block_size: 1,
                }),
            ),
            (Tube::from_string(String::from(""), 4), None),
        ];
        for test in tests {
            let result = test.0.get_top_colour();
            match result {
                Some(col_pos) => {
                    assert!(
                        test.1.is_some(),
                        "expected a None result, but got {}: {}",
                        col_pos.pos,
                        col_pos.colour
                    );
                    let expected = test.1.unwrap();
                    assert_eq!(
                        col_pos.colour, expected.colour,
                        "colours of ColourPos do not match. Expected = {}, got = {}",
                        expected.colour, col_pos.colour
                    );
                    assert_eq!(
                        col_pos.pos, expected.pos,
                        "position of ColourPos does not match. Expected = {}, got = {}",
                        expected.pos, col_pos.pos
                    );
                    assert_eq!(
                        col_pos.block_size, expected.block_size,
                        "block_size of ColourPos does not match. Expected = {}, got = {}",
                        expected.block_size, col_pos.block_size
                    );
                }
                None => {
                    assert!(
                        test.1.is_none(),
                        "got None ColourPos result for tube {}",
                        test.0.tube_number
                    );
                }
            }
        }
    }

    #[test]
    fn test_pour_from() {
        let tests = vec![
            (
                String::from("red, purple, blue, green"),
                Move {
                    tube_from: 0,
                    tube_to: 1,
                    colour: "red".to_string(),
                    quantity: 1,
                },
                Tube::from_string(String::from("purple, blue, green"), 0),
            ),
            (
                String::from("red, red, blue, green"),
                Move {
                    tube_from: 0,
                    tube_to: 1,
                    colour: "red".to_string(),
                    quantity: 1,
                },
                Tube::from_string(String::from("red, blue, green"), 1),
            ),
            (
                String::from("red, red, blue, green"),
                Move {
                    tube_from: 0,
                    tube_to: 1,
                    colour: "red".to_string(),
                    quantity: 2,
                },
                Tube::from_string(String::from("blue, green"), 2),
            ),
            (
                String::from("red, red, red"),
                Move {
                    tube_from: 0,
                    tube_to: 1,
                    colour: "red".to_string(),
                    quantity: 3,
                },
                Tube::from_string(String::from(""), 3),
            ),
            (
                String::from("red, red, blue"),
                Move {
                    tube_from: 0,
                    tube_to: 1,
                    colour: "red".to_string(),
                    quantity: 2,
                },
                Tube::from_string(String::from("empty, empty, empty, blue"), 4),
            ),
        ];

        for (idx, test) in tests.iter().enumerate() {
            let mut result = Tube::from_string(test.0.to_owned(), idx);
            result.pour_from(&test.1);
            test_tube(&result, &test.2);
        }
    }

    #[test]
    fn test_pour_to() {
        let tests = vec![
            (
                String::from(""),
                Move {
                    tube_from: 1,
                    tube_to: 0,
                    colour: "red".to_string(),
                    quantity: 1,
                },
                Tube::from_string(String::from("empty, empty, empty, red"), 0),
            ),
            (
                String::from("red"),
                Move {
                    tube_from: 1,
                    tube_to: 0,
                    colour: "red".to_string(),
                    quantity: 1,
                },
                Tube::from_string(String::from("empty, empty, red, red"), 1),
            ),
            (
                String::from("blue, red"),
                Move {
                    tube_from: 1,
                    tube_to: 0,
                    colour: "red".to_string(),
                    quantity: 1,
                },
                Tube::from_string(String::from("empty, red, blue, red"), 2),
            ),
            (
                String::from("blue, red"),
                Move {
                    tube_from: 1,
                    tube_to: 0,
                    colour: "red".to_string(),
                    quantity: 2,
                },
                Tube::from_string(String::from("red, red, blue, red"), 3),
            ),
        ];

        for (idx, test) in tests.iter().enumerate() {
            let mut result = Tube::from_string(test.0.to_owned(), idx);
            result.pour_to(&test.1);
            test_tube(&result, &test.2);
        }
    }

    #[test]
    fn test_validate_move_from() {
        let tests = vec![
            (
                String::from("red"),
                Move {
                    tube_from: 0,
                    tube_to: 1,
                    colour: "red".to_string(),
                    quantity: 1,
                },
                true,
            ),
            (
                String::from("blue"),
                Move {
                    tube_from: 0,
                    tube_to: 1,
                    colour: "red".to_string(),
                    quantity: 1,
                },
                false,
            ),
            (
                String::from("red"),
                Move {
                    tube_from: 0,
                    tube_to: 1,
                    colour: "red".to_string(),
                    quantity: 2,
                },
                false,
            ),
            (
                String::from("red, red, red"),
                Move {
                    tube_from: 0,
                    tube_to: 1,
                    colour: "red".to_string(),
                    quantity: 1,
                },
                true,
            ),
            (
                String::from("red, red, red, red"),
                Move {
                    tube_from: 0,
                    tube_to: 1,
                    colour: "red".to_string(),
                    quantity: 1,
                },
                true,
            ),
            (
                String::from("red, red, red, red"),
                Move {
                    tube_from: 0,
                    tube_to: 1,
                    colour: "red".to_string(),
                    quantity: 4,
                },
                true,
            ),
        ];

        for test in tests {
            let tube = Tube::from_string(test.0, 0);
            let result = tube.is_valid_move_from(&test.1);
            assert_eq!(
                result, test.2,
                "validate_move_from wrong result for {} from tube {}. Expected = {}, got = {}",
                test.1, tube, test.2, result
            );
        }
    }

    #[test]
    fn test_validate_move_to() {
        let tests = vec![
            (
                String::from("red"),
                Move {
                    tube_from: 1,
                    tube_to: 0,
                    colour: "red".to_string(),
                    quantity: 1,
                },
                true,
            ),
            (
                String::from("blue"),
                Move {
                    tube_from: 1,
                    tube_to: 0,
                    colour: "red".to_string(),
                    quantity: 1,
                },
                false,
            ),
            (
                String::from("red"),
                Move {
                    tube_from: 1,
                    tube_to: 0,
                    colour: "red".to_string(),
                    quantity: 2,
                },
                true,
            ),
            (
                String::from("red"),
                Move {
                    tube_from: 1,
                    tube_to: 0,
                    colour: "red".to_string(),
                    quantity: 3,
                },
                true,
            ),
            (
                String::from("red"),
                Move {
                    tube_from: 1,
                    tube_to: 0,
                    colour: "red".to_string(),
                    quantity: 4,
                },
                false,
            ),
            (
                String::from(""),
                Move {
                    tube_from: 1,
                    tube_to: 0,
                    colour: "red".to_string(),
                    quantity: 1,
                },
                true,
            ),
            (
                String::from("red, red, red, red"),
                Move {
                    tube_from: 1,
                    tube_to: 0,
                    colour: "red".to_string(),
                    quantity: 1,
                },
                false,
            ),
            (
                String::from("red, red, red, red"),
                Move {
                    tube_from: 1,
                    tube_to: 0,
                    colour: "red".to_string(),
                    quantity: 4,
                },
                false,
            ),
            (
                String::from("red, red, red"),
                Move {
                    tube_from: 1,
                    tube_to: 0,
                    colour: "red".to_string(),
                    quantity: 2,
                },
                false,
            ),
            (
                String::from("blue, red, red"),
                Move {
                    tube_from: 1,
                    tube_to: 0,
                    colour: "red".to_string(),
                    quantity: 1,
                },
                false,
            ),
        ];

        for test in tests {
            let tube = Tube::from_string(test.0, 0);
            let result = tube.is_valid_move_to(&test.1);
            assert_eq!(
                result, test.2,
                "validate_move_to wrong result for {} from tube {}. Expected = {}, got = {}",
                test.1, tube, test.2, result
            );
        }
    }

    fn test_tube(test_result: &Tube, expected: &Tube) {
        assert_eq!(
            test_result.contents, expected.contents,
            "tube contents are not the same. Expected = {}, got = {}",
            expected, test_result
        );
        assert_eq!(
            test_result.tube_number, expected.tube_number,
            "tube number not the same. Expected = {}, got = {}",
            expected.tube_number, test_result.tube_number
        );
    }
}
