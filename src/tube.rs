use std::fmt::Display;

use crate::TUBE_SIZE;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Colour {
    Empty,
    Red,
    Blue,
    Green,
    Purple,
}

impl Colour {
    pub fn from_string(colour: &str) -> Colour {
        match colour.trim().to_lowercase().as_str() {
            "red" => Colour::Red,
            "blue" => Colour::Blue,
            "green" => Colour::Green,
            "purple" => Colour::Purple,
            _ => Colour::Empty,
        }
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Colour::Empty => write!(f, "Empty"),
            Colour::Red => write!(f, "Red"),
            Colour::Blue => write!(f, "Blue"),
            Colour::Green => write!(f, "Green"),
            Colour::Purple => write!(f, "Purple"),
        }
    }
}

pub struct ColourPos {
    pub colour: Colour,
    pub pos: usize,
}

pub struct Tube {
    contents: Vec<Colour>,
    tube_number: usize,
}

impl Tube {
    pub fn from_string(string_colours: String, tube_number: usize) -> Tube {
        let mut colours = Vec::with_capacity(TUBE_SIZE);
        let vec_string_colours: Vec<&str> = string_colours.split(',').collect();
        // Add empty cells where there is no string colour supplied
        for _ in vec_string_colours.len()..TUBE_SIZE {
            colours.push(Colour::Empty);
        }
        // Add the colours from the string. Note that any unmatched strings will get set as Empty.
        for str_col in vec_string_colours {
            colours.push(Colour::from_string(str_col));
        }

        Tube {
            contents: colours,
            tube_number,
        }
    }

    pub fn from_string_vec(colours: Vec<String>) -> Tube {
        unimplemented!()
    }

    fn get_top_colour(&self) -> Option<ColourPos> {
        for (pos, colour) in self.contents.iter().enumerate() {
            if colour != &Colour::Empty {
                return Some(ColourPos {
                    colour: *colour,
                    pos,
                });
            };
        }
        None
    }
}

impl Display for Tube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        let mut colours = Vec::new();

        for colour in self.contents.iter() {
            colours.push(format!("{}", colour));
        }

        out.push_str(format!("{}: (", self.tube_number).as_str());
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
                    contents: vec![Colour::Red, Colour::Red, Colour::Blue, Colour::Green],
                    tube_number: 1,
                },
            ),
            (
                String::from("empty, red, blue, green"),
                Tube {
                    contents: vec![Colour::Empty, Colour::Red, Colour::Blue, Colour::Green],
                    tube_number: 2,
                },
            ),
            (
                String::from("red, blue, green"),
                Tube {
                    contents: vec![Colour::Empty, Colour::Red, Colour::Blue, Colour::Green],
                    tube_number: 3,
                },
            ),
            (
                String::from("blue, green"),
                Tube {
                    contents: vec![Colour::Empty, Colour::Empty, Colour::Blue, Colour::Green],
                    tube_number: 4,
                },
            ),
            (
                String::from("blue, green, unknown"),
                Tube {
                    contents: vec![Colour::Empty, Colour::Blue, Colour::Green, Colour::Empty],
                    tube_number: 5,
                },
            ),
            (
                String::from("RED, rEd, Blue    ,    Green      "),
                Tube {
                    contents: vec![Colour::Red, Colour::Red, Colour::Blue, Colour::Green],
                    tube_number: 6,
                },
            ),
        ];

        for (idx, test) in string_tests.into_iter().enumerate() {
            let result = Tube::from_string(test.0, idx + 1);
            test_tube(&result, &test.1);
        }
    }

    #[test]
    fn test_top_colour() {
        let tests = vec![
            (
                Tube::from_string(String::from("red, red, blue, green"), 1),
                Some(ColourPos {
                    colour: Colour::Red,
                    pos: 0,
                }),
            ),
            (
                Tube::from_string(String::from("empty, red, blue, green"), 2),
                Some(ColourPos {
                    colour: Colour::Red,
                    pos: 1,
                }),
            ),
            (
                Tube::from_string(String::from("red, blue, green"), 3),
                Some(ColourPos {
                    colour: Colour::Red,
                    pos: 1,
                }),
            ),
            (Tube::from_string(String::from(""), 4), None),
        ];
        for test in tests {
            let result = test.0.get_top_colour();
            match result {
                Some(col_pos) => {
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
