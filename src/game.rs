use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::{tube::Tube, TUBE_SIZE};

#[derive(Default, Clone)]
pub struct Game {
    pub tubes: Vec<Tube>,
    pub moves: HashMap<usize, Move>,
    pub current_move: usize,
    pub colours: HashSet<String>,
}

impl Game {
    pub fn init_tubes(&mut self, num_of_tubes: usize) {
        if num_of_tubes < 4 {
            panic!("Must have at least 4 tubes to play a valid game!");
        }
        let mut tubes = Vec::with_capacity(num_of_tubes);
        for idx in 0..num_of_tubes {
            tubes.push(Tube::from_string(String::from(""), idx));
        }
        self.tubes = tubes;
    }

    pub fn init_tube_contents(&mut self, tube_num: usize, contents: String) {
        self.tubes[tube_num] = Tube::from_string(contents, tube_num);
        let colours: HashSet<String> = self.tubes[tube_num]
            .contents
            .iter()
            .filter_map(|x| x.clone())
            .collect();
        self.colours.extend(colours);
    }

    pub fn validate_setup(&self) -> bool {
        if self.tubes.len() - 2 != self.colours.len() {
            return false;
        }
        let mut colour_counts: HashMap<String, usize> = HashMap::new();
        for tube in &self.tubes {
            for col in &tube.contents {
                if col.is_some() {
                    let col = col.as_ref().unwrap();
                    match colour_counts.get(col) {
                        Some(count) => colour_counts.insert(col.clone(), count + 1),
                        None => colour_counts.insert(col.clone(), 1),
                    };
                }
            }
        }

        colour_counts
            .values()
            .all(|&expected| expected == TUBE_SIZE)
    }

    pub fn validate_move(&self, a_move: &Move) -> bool {
        let from_tube = &self.tubes[a_move.tube_from];
        let to_tube = &self.tubes[a_move.tube_to];
        from_tube.is_valid_move_from(a_move) && to_tube.is_valid_move_to(a_move)
    }

    pub fn make_move(&mut self, a_move: &Move) {
        if !self.validate_move(a_move) {
            return;
        }
        self.tubes[a_move.tube_from].pour_from(a_move);
        self.tubes[a_move.tube_to].pour_to(a_move);
        self.current_move += 1;
        self.moves.insert(self.current_move, a_move.clone());
    }

    pub fn is_game_complete(&self) -> bool {
        self.tubes
            .iter()
            .all(|tube| tube.is_tube_all_same_contents())
    }

    pub fn get_all_moves_string(&self) -> String {
        let mut all_moves = String::new();
        for (move_num, a_move) in self.moves.iter().sorted_by_key(|x| x.0) {
            all_moves.push_str(format!("{} : ({})\n", move_num, a_move).as_str());
        }
        all_moves
    }

    pub fn print_colour(&self, requested_colour: &str) -> String {
        let mut requested_colour = requested_colour.to_string();
        match self.colours.contains(&requested_colour) {
            true => requested_colour.remove(0).to_uppercase().to_string() + &requested_colour,
            false => "Empty".to_string(),
        }
    }

    pub fn is_num_of_colours_valid(&self) -> bool {
        self.colours.len() == self.tubes.len() - 2
    }

    pub fn get_number_of_blocks(&self) -> usize {
        let mut blocks = 0;
        for tube in self.tubes.iter() {
            let mut current_colour: Option<String> = None;
            for segment in tube.contents.iter() {
                match segment {
                    Some(col) => {
                        if current_colour.is_none() {
                            current_colour = Some(col.clone());
                            continue;
                        } else if col == &current_colour.clone().unwrap() {
                            continue;
                        } else {
                            blocks += 1;
                            current_colour = Some(col.clone());
                        }
                    }
                    None => {
                        if current_colour.is_none() {
                            continue;
                        } else {
                            current_colour = None;
                            blocks += 1;
                        }
                    }
                }
            }
            if current_colour.is_some() {
                blocks += 1;
            }
        }
        blocks
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for tube in &self.tubes {
            out.push_str(format!("{}", tube).as_str());
            out.push('\n');
        }
        write!(f, "{}", out)
    }
}

#[derive(Clone)]
pub struct Move {
    pub tube_from: usize,
    pub tube_to: usize,
    pub colour: String,
    pub quantity: usize,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = format!(
            "{} -> {}: {} x {}",
            self.tube_from + 1,
            self.tube_to + 1,
            self.colour,
            self.quantity
        );

        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_game() {
        let mut game = Game::default();
        game.init_tubes(4);
        game.init_tube_contents(0, String::from("red, blue, green, red"));
        game.init_tube_contents(1, String::from("green, blue, red, purple"));

        let expected = Game {
            tubes: vec![
                Tube::from_string(String::from("red, blue, green, red"), 0),
                Tube::from_string(String::from("green, blue, red, purple"), 1),
                Tube::from_string_vec(vec![None; 4], 2),
                Tube::from_string_vec(vec![None; 4], 3),
            ],
            moves: HashMap::new(),
            current_move: 0,
            colours: HashSet::from([
                "red".to_string(),
                "green".to_string(),
                "blue".to_string(),
                "purple".to_string(),
            ]),
        };
        test_all_tubes(&game.tubes, &expected.tubes);
        assert_eq!(
            game.current_move, 0,
            "current move wrong value. Expected = {}, got = {}",
            0, game.current_move
        );
        assert!(game.moves.is_empty(), "moves are not empty");
        assert_eq!(
            game.colours, expected.colours,
            "Colours hashset is not the same. Expected = {:?}, got = {:?}",
            expected.colours, game.colours
        );
    }

    #[test]
    fn test_post_setup_validation() {
        let num_of_tubes: usize = 4;
        let tests: Vec<(Vec<String>, bool)> = vec![
            (
                vec![
                    String::from("red, red, red, red"),
                    String::from("blue, blue, blue, blue"),
                ],
                true,
            ),
            (
                vec![
                    String::from("red, red, red, red"),
                    String::from("blue, blue, blue, red"),
                ],
                false,
            ),
            (
                vec![
                    String::from("red, red, red, red"),
                    String::from("blue, blue, blue, blue"),
                    String::from("green"),
                ],
                false,
            ),
            (
                vec![
                    String::from("red, blue"),
                    String::from("red, blue"),
                    String::from("red, blue"),
                    String::from("blue, red"),
                ],
                true,
            ),
            (
                vec![
                    String::from("red, red, blue"),
                    String::from("blue, red, red, blue"),
                ],
                false,
            ),
        ];
        for test in tests {
            let mut game = Game::default();
            game.init_tubes(num_of_tubes);
            for (idx, init_tube) in test.0.into_iter().enumerate() {
                game.init_tube_contents(idx, init_tube);
            }
            let val_res = game.validate_setup();
            assert_eq!(
                val_res, test.1,
                "game setup validate incorrect. Expected: {}, got: {}",
                test.1, val_res
            );
        }
    }

    #[test]
    fn test_move_validation() {
        let num_of_tubes: usize = 4;
        let tests: Vec<(Vec<String>, Move, bool)> = vec![
            (
                vec![
                    String::from("blue, red, blue, red"),
                    String::from("red, red"),
                    String::from("blue, blue"),
                ],
                Move {
                    tube_from: 0,
                    tube_to: 1,
                    colour: "red".to_string(),
                    quantity: 1,
                },
                false,
            ),
            (
                vec![
                    String::from("blue, red, blue, red"),
                    String::from("red, red"),
                    String::from("blue, blue"),
                ],
                Move {
                    tube_from: 0,
                    tube_to: 2,
                    colour: "blue".to_string(),
                    quantity: 1,
                },
                true,
            ),
            (
                vec![
                    String::from("blue, red, blue, red"),
                    String::from("red, red"),
                    String::from("blue, blue"),
                ],
                Move {
                    tube_from: 0,
                    tube_to: 2,
                    colour: "blue".to_string(),
                    quantity: 3,
                },
                false,
            ),
            (
                vec![
                    String::from("blue, red, blue, red"),
                    String::from("red, red"),
                    String::from("blue, blue"),
                ],
                Move {
                    tube_from: 0,
                    tube_to: 2,
                    colour: "blue".to_string(),
                    quantity: 2,
                },
                false,
            ),
            (
                vec![
                    String::from("blue, red, blue, red"),
                    String::from("red, red"),
                    String::from("blue, blue"),
                ],
                Move {
                    tube_from: 1,
                    tube_to: 3,
                    colour: "red".to_string(),
                    quantity: 1,
                },
                true,
            ),
            (
                vec![
                    String::from("blue, red, blue, red"),
                    String::from("red, red"),
                    String::from("blue, blue"),
                ],
                Move {
                    tube_from: 1,
                    tube_to: 3,
                    colour: "red".to_string(),
                    quantity: 2,
                },
                true,
            ),
            (
                vec![
                    String::from("blue, red, blue, red"),
                    String::from("red, red"),
                    String::from("blue, blue"),
                ],
                Move {
                    tube_from: 1,
                    tube_to: 3,
                    colour: "red".to_string(),
                    quantity: 3,
                },
                false,
            ),
        ];
        for test in tests {
            let mut game = Game::default();
            game.init_tubes(num_of_tubes);
            for (idx, init_tube) in test.0.into_iter().enumerate() {
                game.init_tube_contents(idx, init_tube);
            }
            let val_res = game.validate_move(&test.1);
            assert_eq!(
                val_res, test.2,
                "game validation incorrect for move: {}. Expected = {}, got = {}",
                test.1, test.2, val_res
            );
        }
    }

    #[test]
    fn test_single_move() {
        // All of these tests performed on games with 4 tubes
        let num_of_tubes: usize = 4;
        let tests: Vec<(Vec<String>, Move, Game)> = vec![
            (
                vec![
                    String::from("blue, red, blue, red"),
                    String::from("red, red"),
                    String::from("blue, blue"),
                ],
                Move {
                    tube_from: 0,
                    tube_to: 2,
                    colour: "blue".to_string(),
                    quantity: 1,
                },
                Game {
                    tubes: vec![
                        Tube {
                            tube_number: 0,
                            contents: vec![
                                None,
                                Some("red".to_string()),
                                Some("blue".to_string()),
                                Some("red".to_string()),
                            ],
                        },
                        Tube {
                            tube_number: 1,
                            contents: vec![
                                None,
                                None,
                                Some("red".to_string()),
                                Some("red".to_string()),
                            ],
                        },
                        Tube {
                            tube_number: 2,
                            contents: vec![
                                None,
                                Some("blue".to_string()),
                                Some("blue".to_string()),
                                Some("blue".to_string()),
                            ],
                        },
                        Tube {
                            tube_number: 3,
                            contents: vec![None; 4],
                        },
                    ],
                    moves: HashMap::from([(
                        1,
                        Move {
                            tube_from: 0,
                            tube_to: 2,
                            colour: "blue".to_string(),
                            quantity: 1,
                        },
                    )]),
                    current_move: 1,
                    colours: vec!["red".to_string(), "blue".to_string()]
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect(),
                },
            ),
            (
                vec![
                    String::from("blue, red, blue, red"),
                    String::from("red, red"),
                    String::from("blue, blue"),
                ],
                Move {
                    tube_from: 1,
                    tube_to: 3,
                    colour: "red".to_string(),
                    quantity: 1,
                },
                Game {
                    tubes: vec![
                        Tube {
                            tube_number: 0,
                            contents: vec![
                                Some("blue".to_string()),
                                Some("red".to_string()),
                                Some("blue".to_string()),
                                Some("red".to_string()),
                            ],
                        },
                        Tube {
                            tube_number: 1,
                            contents: vec![None, None, None, Some("red".to_string())],
                        },
                        Tube {
                            tube_number: 2,
                            contents: vec![
                                None,
                                None,
                                Some("blue".to_string()),
                                Some("blue".to_string()),
                            ],
                        },
                        Tube {
                            tube_number: 3,
                            contents: vec![None, None, None, Some("red".to_string())],
                        },
                    ],
                    moves: HashMap::from([(
                        1,
                        Move {
                            tube_from: 1,
                            tube_to: 3,
                            colour: "red".to_string(),
                            quantity: 1,
                        },
                    )]),
                    current_move: 1,
                    colours: vec!["red".to_string(), "blue".to_string()]
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect(),
                },
            ),
            (
                vec![
                    String::from("blue, red, blue, red"),
                    String::from("red, red"),
                    String::from("blue, blue"),
                ],
                Move {
                    tube_from: 1,
                    tube_to: 3,
                    colour: "red".to_string(),
                    quantity: 2,
                },
                Game {
                    tubes: vec![
                        Tube {
                            tube_number: 0,
                            contents: vec![
                                Some("blue".to_string()),
                                Some("red".to_string()),
                                Some("blue".to_string()),
                                Some("red".to_string()),
                            ],
                        },
                        Tube {
                            tube_number: 1,
                            contents: vec![None, None, None, None],
                        },
                        Tube {
                            tube_number: 2,
                            contents: vec![
                                None,
                                None,
                                Some("blue".to_string()),
                                Some("blue".to_string()),
                            ],
                        },
                        Tube {
                            tube_number: 3,
                            contents: vec![
                                None,
                                None,
                                Some("red".to_string()),
                                Some("red".to_string()),
                            ],
                        },
                    ],
                    moves: HashMap::from([(
                        1,
                        Move {
                            tube_from: 1,
                            tube_to: 3,
                            colour: "red".to_string(),
                            quantity: 2,
                        },
                    )]),
                    current_move: 1,
                    colours: vec!["red".to_string(), "blue".to_string()]
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect(),
                },
            ),
        ];

        for test in tests {
            let mut game = Game::default();
            game.init_tubes(num_of_tubes);
            for (idx, init_tube) in test.0.into_iter().enumerate() {
                game.init_tube_contents(idx, init_tube);
            }
            game.make_move(&test.1);
            test_all_tubes(&game.tubes, &test.2.tubes);
            assert_eq!(
                game.current_move, 1,
                "current move not expected value. Expected = 1, got = {}",
                game.current_move
            );
            assert_eq!(
                game.moves.len(),
                1,
                "game moves not expected len. Expected = 1, got = {}",
                game.moves.len()
            );
            match game.moves.get(&1_usize) {
                Some(move1) => test_move(move1, &test.1),
                None => panic!("Did not find move 1"),
            }
        }
    }

    #[test]
    fn test_is_game_complete() {
        let tests = vec![
            (
                Game {
                    tubes: vec![Tube {
                        contents: vec![
                            Some("red".to_string()),
                            Some("red".to_string()),
                            Some("red".to_string()),
                            Some("red".to_string()),
                        ],
                        tube_number: 0,
                    }],
                    moves: HashMap::new(),
                    current_move: 0,
                    colours: HashSet::from(["red".to_string()]),
                },
                true,
            ),
            (
                Game {
                    tubes: vec![Tube {
                        contents: vec![
                            Some("blue".to_string()),
                            Some("red".to_string()),
                            Some("red".to_string()),
                            Some("red".to_string()),
                        ],
                        tube_number: 0,
                    }],
                    moves: HashMap::new(),
                    current_move: 0,
                    colours: HashSet::from(["red".to_string(), "blue".to_string()]),
                },
                false,
            ),
            (
                Game {
                    tubes: vec![Tube {
                        contents: vec![None, None, None, None],
                        tube_number: 0,
                    }],
                    moves: HashMap::new(),
                    current_move: 0,
                    colours: HashSet::new(),
                },
                true,
            ),
            (
                Game {
                    tubes: vec![Tube {
                        contents: vec![None, None, None, Some("red".to_string())],
                        tube_number: 0,
                    }],
                    moves: HashMap::new(),
                    current_move: 0,
                    colours: HashSet::from(["red".to_string()]),
                },
                false,
            ),
            (
                Game {
                    tubes: vec![
                        Tube {
                            contents: vec![
                                Some("blue".to_string()),
                                Some("red".to_string()),
                                Some("red".to_string()),
                                Some("red".to_string()),
                            ],
                            tube_number: 0,
                        },
                        Tube {
                            contents: vec![
                                Some("blue".to_string()),
                                Some("blue".to_string()),
                                Some("blue".to_string()),
                                Some("blue".to_string()),
                            ],
                            tube_number: 1,
                        },
                    ],
                    moves: HashMap::new(),
                    current_move: 0,
                    colours: HashSet::from(["red".to_string(), "blue".to_string()]),
                },
                false,
            ),
        ];

        for test in tests {
            let result = test.0.is_game_complete();
            assert_eq!(
                result, test.1,
                "Game complete check incorrect. Expected: {}, got = {}",
                test.1, result
            );
        }
    }

    fn test_all_tubes(result: &[Tube], expected: &[Tube]) {
        assert_eq!(
            result.len(),
            expected.len(),
            "different number of tubes. Expected = {}, got = {}",
            result.len(),
            expected.len()
        );
        for (idx, expected_tube) in expected.iter().enumerate() {
            test_tube(&result[idx], expected_tube);
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

    fn test_move(test_result: &Move, expected: &Move) {
        assert_eq!(
            test_result.tube_from, expected.tube_from,
            "Move.tube_from different. Expected = {}, got = {}",
            expected.tube_from, test_result.tube_from
        );
        assert_eq!(
            test_result.tube_to, expected.tube_to,
            "Move.tube_to different. Expected = {}, got = {}",
            expected.tube_to, test_result.tube_to
        );
        assert_eq!(
            test_result.colour, expected.colour,
            "Move.colour different. Expected = {}, got = {}",
            expected.colour, test_result.colour
        );
        assert_eq!(
            test_result.quantity, expected.quantity,
            "Move.colour different. Expected = {}, got = {}",
            expected.quantity, test_result.quantity
        );
    }
}
