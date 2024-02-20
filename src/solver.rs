use std::cmp::min;

use crate::{
    game::{Game, Move},
    TUBE_SIZE,
};

pub struct Solver {
    states: Vec<Vec<Game>>,
    current_state: Game,
    current_block_count: usize,
}

impl Solver {
    pub fn new(current_state: &Game) -> Solver {
        let number_of_blocks = current_state.get_number_of_blocks();
        let mut states: Vec<Vec<Game>> = Vec::new();
        if number_of_blocks + 2 == current_state.tubes.len() {
            return Solver {
                states,
                current_state: current_state.clone(),
                current_block_count: number_of_blocks,
            };
        }
        for _ in 0..number_of_blocks - (current_state.tubes.len() - 2) {
            states.push(Vec::new());
        }
        states[0].push(current_state.clone());

        Solver {
            states,
            current_state: current_state.clone(),
            current_block_count: number_of_blocks,
        }
    }

    fn get_possible_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for (from_idx, from_tube) in self.current_state.tubes.iter().enumerate() {
            let from_top_colour = from_tube.get_top_colour();
            if from_top_colour.is_none() {
                break;
            }
            let from_top_colour = from_top_colour.unwrap();
            for (to_idx, to_tube) in self.current_state.tubes.iter().enumerate() {
                if from_idx == to_idx {
                    continue;
                }
                let to_top_colour = to_tube.get_top_colour();
                if to_top_colour.is_none() {
                    // Do not allow moves where you are emptying a tube and the destination tube is already empty.
                    if TUBE_SIZE - from_top_colour.block_size == from_top_colour.pos {
                        continue;
                    }

                    moves.push(Move {
                        tube_from: from_idx,
                        tube_to: to_idx,
                        colour: from_top_colour.colour.clone(),
                        quantity: from_top_colour.block_size,
                    });
                    continue;
                }
                let to_top_colour = to_top_colour.unwrap();
                if to_top_colour.colour == from_top_colour.colour {
                    moves.push(Move {
                        tube_from: from_idx,
                        tube_to: to_idx,
                        colour: from_top_colour.colour.clone(),
                        quantity: min(from_top_colour.block_size, to_top_colour.pos),
                    })
                }
            }
        }

        moves
    }

    fn does_move_reduce_block_count(&self, possible_move: &Move) -> bool {
        self.current_block_count > self.peek_move(possible_move).get_number_of_blocks()
    }

    fn peek_move(&self, possible_move: &Move) -> Game {
        if !self.current_state.validate_move(possible_move) {
            return self.current_state.clone();
        }
        let mut peek_game = self.current_state.clone();
        peek_game.make_move(possible_move);
        peek_game
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::tube::Tube;

    use super::*;

    #[test]
    fn test_get_number_of_blocks() {
        let num_of_tubes = 4;
        let tests: Vec<(Vec<String>, usize)> = vec![
            (
                vec![
                    String::from("red,blue,red,blue"),
                    String::from("blue,red,blue,red"),
                ],
                8,
            ),
            (
                vec![
                    String::from("red,red,red,red"),
                    String::from("blue,blue,blue,blue"),
                ],
                2,
            ),
            (
                vec![
                    String::from("red, red"),
                    String::from("blue, blue"),
                    String::from("red, blue, blue"),
                    String::from("red"),
                ],
                5,
            ),
            (vec![String::from("")], 0),
        ];

        for test in tests {
            let game = initialise_game(test.0, num_of_tubes);
            let num_of_blocks = game.get_number_of_blocks();
            assert_eq!(
                num_of_blocks, test.1,
                "incorrect number of blocks returned. Expected: {}, got: {}",
                test.1, num_of_blocks
            );
        }
    }

    #[test]
    fn test_solver_init() {
        let num_of_tubes = 4;
        // Vec<String> = initial tube setup
        // usize = x = initial list of number of moves which do decrease the number of blocks
        // Game = initial status of the game in states[0][0]
        let tests: Vec<(Vec<String>, usize, Game)> = vec![
            (
                vec![
                    String::from("red,red,blue,blue"),
                    String::from("blue,blue,red,red"),
                ],
                2,
                Game {
                    tubes: vec![
                        Tube::from_string(String::from("red,red,blue,blue"), 0),
                        Tube::from_string(String::from("blue,blue,red,red"), 1),
                        Tube::from_string_vec(vec![None; 4], 2),
                        Tube::from_string_vec(vec![None; 4], 3),
                    ],
                    moves: HashMap::new(),
                    current_move: 0,
                    colours: HashSet::from(["red".to_string(), "blue".to_string()]),
                },
            ),
            (
                vec![
                    String::from("red, blue, green"),
                    String::from("red, blue, green"),
                ],
                4,
                Game {
                    tubes: vec![
                        Tube::from_string(String::from("red, blue, green"), 0),
                        Tube::from_string(String::from("red, blue, green"), 1),
                        Tube::from_string_vec(vec![None; 4], 2),
                        Tube::from_string_vec(vec![None; 4], 3),
                    ],
                    moves: HashMap::new(),
                    current_move: 0,
                    colours: HashSet::from([
                        "red".to_string(),
                        "blue".to_string(),
                        "green".to_string(),
                    ]),
                },
            ),
        ];
        for test in tests {
            let game = initialise_game(test.0, num_of_tubes);
            let solver = Solver::new(&game);
            assert_eq!(
                solver.states.len(),
                test.1,
                "solver has incorrect size in x direction. Expected: {}, got: {}",
                test.1,
                solver.states.len()
            );
            for (idx, state) in solver.states.iter().enumerate() {
                if idx == 0 {
                    assert_eq!(state.len(), 1, "solver states for x = {} has incorrect size in y direction. Expected: 1, got: {}", idx, state.len());
                    let state_0_0 = &state[idx];
                    test_all_tubes(&state_0_0.tubes, &test.2.tubes);
                    assert_eq!(
                        state_0_0.current_move, 0,
                        "current move wrong value. Expected = {}, got = {}",
                        0, state_0_0.current_move
                    );
                    assert!(state_0_0.moves.is_empty(), "moves are not empty");
                    assert_eq!(
                        state_0_0.colours, test.2.colours,
                        "Colours hashset is not the same. Expected = {:?}, got = {:?}",
                        test.2.colours, state_0_0.colours
                    );
                } else {
                    assert_eq!(state.len(), 0, "solver states for x = {} has incorrect size in y direction. Expected: 0, got: {}", idx, state.len());
                }
            }
        }
    }

    #[test]
    fn test_get_possible_moves() {
        let num_of_tubes = 4;
        let tests: Vec<(Vec<String>, Vec<Move>)> = vec![
            (
                vec![
                    String::from("red, red, red"),
                    String::from("blue, blue, blue, blue"),
                    String::from("red"),
                ],
                vec![
                    Move {
                        tube_from: 0,
                        tube_to: 2,
                        colour: String::from("red"),
                        quantity: 3,
                    },
                    Move {
                        tube_from: 2,
                        tube_to: 0,
                        colour: String::from("red"),
                        quantity: 1,
                    },
                ],
            ),
            (
                vec![
                    String::from("red, red, red"),
                    String::from("blue, blue, blue, blue"),
                ],
                Vec::new(),
            ),
            (
                vec![
                    String::from("red, red"),
                    String::from("blue, blue, blue"),
                    String::from("red, red"),
                    String::from("blue"),
                ],
                vec![
                    Move {
                        tube_from: 0,
                        tube_to: 2,
                        colour: String::from("red"),
                        quantity: 2,
                    },
                    Move {
                        tube_from: 2,
                        tube_to: 0,
                        colour: String::from("red"),
                        quantity: 2,
                    },
                    Move {
                        tube_from: 1,
                        tube_to: 3,
                        colour: String::from("blue"),
                        quantity: 3,
                    },
                    Move {
                        tube_from: 3,
                        tube_to: 1,
                        colour: String::from("blue"),
                        quantity: 1,
                    },
                ],
            ),
            (
                vec![
                    String::from("red, red, red"),
                    String::from("red, blue, blue"),
                    String::from("blue, blue"),
                ],
                vec![
                    Move {
                        tube_from: 0,
                        tube_to: 1,
                        colour: String::from("red"),
                        quantity: 1,
                    },
                    Move {
                        tube_from: 1,
                        tube_to: 0,
                        colour: String::from("red"),
                        quantity: 1,
                    },
                    Move {
                        tube_from: 1,
                        tube_to: 3,
                        colour: String::from("red"),
                        quantity: 1,
                    },
                ],
            ),
        ];
        for test in tests {
            let game = initialise_game(test.0, num_of_tubes);
            let solver = Solver::new(&game);
            let possible_moves = solver.get_possible_moves();
            assert_eq!(
                possible_moves.len(),
                test.1.len(),
                "possible moves wrong length. Expected: {}, got: {}",
                test.1.len(),
                possible_moves.len()
            );
            for expected_move in test.1.iter() {
                let mut found = false;
                for possible_move in possible_moves.iter() {
                    if possible_move.tube_from == expected_move.tube_from
                        && possible_move.tube_to == expected_move.tube_to
                    {
                        test_move(possible_move, expected_move);
                        found = true;
                        break;
                    }
                }
                if !found {
                    panic!("expected move cannot be found: {}", expected_move);
                }
            }
        }
    }

    #[test]
    fn test_does_move_reduce_block_count() {
        let num_of_tubes = 4;
        let tests: Vec<(Vec<String>, Move, bool)> = vec![
            (
                vec![
                    String::from("red, red"),
                    String::from("red, red"),
                    String::from("blue, blue, blue, blue"),
                ],
                Move {
                    tube_from: 0,
                    tube_to: 1,
                    colour: String::from("red"),
                    quantity: 2,
                },
                true,
            ),
            (
                vec![
                    String::from("red, red, red"),
                    String::from("red, blue"),
                    String::from("blue, blue, blue"),
                ],
                Move {
                    tube_from: 0,
                    tube_to: 1,
                    colour: String::from("red"),
                    quantity: 1,
                },
                false,
            ),
            (
                vec![
                    String::from("red, red, red"),
                    String::from("red, blue"),
                    String::from("blue, blue, blue"),
                ],
                Move {
                    tube_from: 1,
                    tube_to: 0,
                    colour: String::from("red"),
                    quantity: 1,
                },
                true,
            ),
            (
                vec![
                    String::from("red, red"),
                    String::from("red, red"),
                    String::from("blue, blue, blue, blue"),
                ],
                Move {
                    tube_from: 0,
                    tube_to: 3,
                    colour: String::from("red"),
                    quantity: 2,
                },
                false,
            ),
        ];
        for test in tests {
            let game = initialise_game(test.0, num_of_tubes);
            let solver = Solver::new(&game);
            let result = solver.does_move_reduce_block_count(&test.1);
            assert_eq!(result, test.2, "does move reduce block count gives incorrect return value. Expected: {}, got: {} for move: {}", test.2, result, test.1);
        }
    }

    fn initialise_game(tube_strings: Vec<String>, num_of_tubes: usize) -> Game {
        let mut game = Game::default();
        game.init_tubes(num_of_tubes);
        for (idx, tube_string) in tube_strings.into_iter().enumerate() {
            game.init_tube_contents(idx, tube_string);
        }
        game
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
