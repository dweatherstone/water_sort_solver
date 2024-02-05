use core::num;
use std::{collections::HashMap, fmt::Display, hint::unreachable_unchecked, iter::Map};

use crate::tube::Tube;

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

pub struct Game {
    pub tubes: Vec<Tube>,
    pub moves: HashMap<usize, Move>,
    pub current_move: usize,
}

impl Game {
    pub fn new(num_of_tubes: usize) -> Game {
        if num_of_tubes < 4 {
            panic!("Must have at least 4 tubes to play a valid game!");
        }
        let mut tubes = Vec::with_capacity(num_of_tubes);
        for idx in 0..num_of_tubes {
            tubes.push(Tube::from_string(String::from(""), idx));
        }

        Game {
            tubes,
            moves: HashMap::new(),
            current_move: 0,
        }
    }

    pub fn init_tube_contents(&mut self, tube_num: usize, contents: String) {
        self.tubes[tube_num] = Tube::from_string(contents, tube_num);
    }

    pub fn validate_move(&self, a_move: &Move) -> bool {
        let from_tube = &self.tubes[a_move.tube_from];
        let to_tube = &self.tubes[a_move.tube_to];
        from_tube.validate_move_from(a_move) && to_tube.validate_move_to(a_move)
    }

    pub fn make_move(&mut self, a_move: &Move) {
        if !self.validate_move(a_move) {
            return;
        }
        self.tubes[a_move.tube_from].pour_from(a_move);
        self.tubes[a_move.tube_to].pour_to(a_move);
        self.current_move += 1;
        self.moves.insert(self.current_move, *a_move);
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

#[derive(Clone, Copy)]
pub struct Move {
    pub tube_from: usize,
    pub tube_to: usize,
    pub colour: Colour,
    pub quantity: usize,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = format!(
            "{} -> {}: {} x {}",
            self.tube_from, self.tube_to, self.colour, self.quantity
        );

        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_game() {
        let mut game = Game::new(4);
        game.init_tube_contents(0, String::from("red, blue, green, purple"));
        game.init_tube_contents(1, String::from("green, blue, red, red"));

        let expected = Game {
            tubes: vec![
                Tube::from_string(String::from("red, blue, green, purple"), 0),
                Tube::from_string(String::from("green, blue, red, red"), 1),
                Tube::from_colour_vec(vec![Colour::Empty; 4], 2),
                Tube::from_colour_vec(vec![Colour::Empty; 4], 3),
            ],
            moves: HashMap::new(),
            current_move: 0,
        };
        test_all_tubes(&game.tubes, &expected.tubes);
        assert_eq!(
            game.current_move, 0,
            "current move wrong value. Expected = {}, got = {}",
            0, game.current_move
        );
        assert!(game.moves.is_empty(), "moves are not empty");
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
                    colour: Colour::Red,
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
                    colour: Colour::Blue,
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
                    colour: Colour::Blue,
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
                    colour: Colour::Blue,
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
                    colour: Colour::Red,
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
                    colour: Colour::Red,
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
                    colour: Colour::Red,
                    quantity: 3,
                },
                false,
            ),
        ];
        for test in tests {
            let mut game = Game::new(num_of_tubes);
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
                    colour: Colour::Blue,
                    quantity: 1,
                },
                Game {
                    tubes: vec![
                        Tube {
                            tube_number: 0,
                            contents: vec![Colour::Empty, Colour::Red, Colour::Blue, Colour::Red],
                        },
                        Tube {
                            tube_number: 1,
                            contents: vec![Colour::Empty, Colour::Empty, Colour::Red, Colour::Red],
                        },
                        Tube {
                            tube_number: 2,
                            contents: vec![Colour::Empty, Colour::Blue, Colour::Blue, Colour::Blue],
                        },
                        Tube {
                            tube_number: 3,
                            contents: vec![Colour::Empty; 4],
                        },
                    ],
                    moves: HashMap::from([(
                        1,
                        Move {
                            tube_from: 0,
                            tube_to: 2,
                            colour: Colour::Blue,
                            quantity: 1,
                        },
                    )]),
                    current_move: 1,
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
                    colour: Colour::Red,
                    quantity: 1,
                },
                Game {
                    tubes: vec![
                        Tube {
                            tube_number: 0,
                            contents: vec![Colour::Blue, Colour::Red, Colour::Blue, Colour::Red],
                        },
                        Tube {
                            tube_number: 1,
                            contents: vec![
                                Colour::Empty,
                                Colour::Empty,
                                Colour::Empty,
                                Colour::Red,
                            ],
                        },
                        Tube {
                            tube_number: 2,
                            contents: vec![
                                Colour::Empty,
                                Colour::Empty,
                                Colour::Blue,
                                Colour::Blue,
                            ],
                        },
                        Tube {
                            tube_number: 3,
                            contents: vec![
                                Colour::Empty,
                                Colour::Empty,
                                Colour::Empty,
                                Colour::Red,
                            ],
                        },
                    ],
                    moves: HashMap::from([(
                        1,
                        Move {
                            tube_from: 1,
                            tube_to: 3,
                            colour: Colour::Red,
                            quantity: 1,
                        },
                    )]),
                    current_move: 1,
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
                    colour: Colour::Red,
                    quantity: 2,
                },
                Game {
                    tubes: vec![
                        Tube {
                            tube_number: 0,
                            contents: vec![Colour::Blue, Colour::Red, Colour::Blue, Colour::Red],
                        },
                        Tube {
                            tube_number: 1,
                            contents: vec![
                                Colour::Empty,
                                Colour::Empty,
                                Colour::Empty,
                                Colour::Empty,
                            ],
                        },
                        Tube {
                            tube_number: 2,
                            contents: vec![
                                Colour::Empty,
                                Colour::Empty,
                                Colour::Blue,
                                Colour::Blue,
                            ],
                        },
                        Tube {
                            tube_number: 3,
                            contents: vec![Colour::Empty, Colour::Empty, Colour::Red, Colour::Red],
                        },
                    ],
                    moves: HashMap::from([(
                        1,
                        Move {
                            tube_from: 1,
                            tube_to: 3,
                            colour: Colour::Red,
                            quantity: 2,
                        },
                    )]),
                    current_move: 1,
                },
            ),
        ];

        for test in tests {
            let mut game = Game::new(num_of_tubes);
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

    fn test_all_tubes(result: &Vec<Tube>, expected: &Vec<Tube>) {
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
