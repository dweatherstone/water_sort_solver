use std::io::{Stdin, Stdout, Write};

use crate::{
    game::{Colour, Game, Move},
    TUBE_SIZE,
};

const FLUSH_ERR_MSG: &str = "should have flushed stdout";
const ERR_MSG_WRITE_ERR_MSG: &str = "should have written an error message";

pub struct Repl {
    stdin: Stdin,
    stdout: Stdout,
    current_state: Game,
}

impl Repl {
    pub fn new(stdin: Stdin, stdout: Stdout, init_state: Game) -> Repl {
        Repl {
            current_state: init_state,
            stdin,
            stdout,
        }
    }

    pub fn start(&mut self) {
        loop {
            write!(self.stdout, "Enter the total number of tubes in the game: ")
                .expect("error writing prompt string");
            self.stdout.flush().expect(FLUSH_ERR_MSG);
            let mut input = String::new();
            if let Err(e) = self.stdin.read_line(&mut input) {
                writeln!(self.stdout, "Error: {e}").expect(ERR_MSG_WRITE_ERR_MSG);
                return;
            }
            let num_of_tubes = match input.trim().parse::<usize>() {
                Ok(tube_num) => tube_num,
                Err(_) => {
                    writeln!(self.stdout, "Unable to parse {} to a number", input);
                    continue;
                }
            };
            self.current_state.init_tubes(num_of_tubes);
            for idx in 0..num_of_tubes {
                write!(self.stdout, "Enter the initial state of tube {}: ", idx + 1)
                    .expect(FLUSH_ERR_MSG);
                self.stdout.flush().expect(FLUSH_ERR_MSG);
                let mut input = String::new();
                if let Err(e) = self.stdin.read_line(&mut input) {
                    writeln!(self.stdout, "Error: {e}").expect(ERR_MSG_WRITE_ERR_MSG);
                    return;
                }
                self.current_state.init_tube_contents(idx, input);
            }
            break;
        }
        writeln!(self.stdout, "Starting state of the game:");
        writeln!(self.stdout, "{}", self.current_state);
    }

    pub fn play(&mut self) {
        let mut is_complete = false;
        while !is_complete {
            write!(
                self.stdout,
                "Enter a move in the format (without quotes): \"<tube_from> <tube_to> <quantity>\": "
            )
            .expect("error writing move prompt string");
            self.stdout.flush().expect(FLUSH_ERR_MSG);
            let mut input = String::new();
            if let Err(e) = self.stdin.read_line(&mut input) {
                writeln!(self.stdout, "Error: {e}").expect(ERR_MSG_WRITE_ERR_MSG);
                continue;
            }
            input = input.trim().to_string();
            match input.as_str() {
                "restart" => {
                    is_complete = true;
                    continue;
                }
                "quit" => {
                    is_complete = true;
                    continue;
                }
                _ => {}
            }
            let move_input = match MoveInput::parse_move(input, &self.current_state) {
                Err(err) => {
                    writeln!(self.stdout, "Unable to parse move: {}", err);
                    continue;
                }
                Ok(move_in) => move_in,
            };
            let tube_from = match self
                .current_state
                .tubes
                .get((move_input.tube_from - 1) as usize)
            {
                Some(tube) => tube.to_owned(),
                None => {
                    writeln!(
                        self.stdout,
                        "Error: Unable to find 'from tube' {}",
                        move_input.tube_from - 1
                    );
                    continue;
                }
            };
            let from_colour = match tube_from.get_top_colour() {
                Some(col) => col.colour,
                None => Colour::Empty,
            };
            let this_move = Move {
                tube_from: (move_input.tube_from - 1) as usize,
                tube_to: (move_input.tube_to - 1) as usize,
                quantity: move_input.quantity as usize,
                colour: from_colour,
            };
            if self.current_state.validate_move(&this_move) {
                self.current_state.make_move(&this_move);
                writeln!(self.stdout, "After move: {}:", &this_move);
                writeln!(self.stdout, "{}", self.current_state);
            } else {
                writeln!(self.stdout, "Move is invalid");
                continue;
            }
            if self.current_state.is_game_complete() {
                is_complete = true;
                writeln!(
                    self.stdout,
                    "Congratulations! You have completed the game! The moves were:"
                );
                writeln!(self.stdout, "{}", self.current_state.get_all_moves_string());
            }
        }
    }
}

struct MoveInput {
    tube_from: i32,
    tube_to: i32,
    quantity: i32,
}

impl MoveInput {
    fn parse_move(move_string: String, game: &Game) -> Result<MoveInput, String> {
        // A move string should be of the format "<tube_from> <tube_to> <quantity>" (i.e. space delimited)
        let string_parts: Vec<&str> = move_string.split(' ').collect();
        if string_parts.len() != 3 {
            return Err(
                "Move must be in the format\"<tube_from> <tube_to> <quantity>\"".to_string(),
            );
        }
        let tube_from = match string_parts[0].parse::<i32>() {
            Ok(entry) => entry,
            Err(e) => return Err("Expected an integer for the 'from tube' value".to_string()),
        };
        let tube_to = match string_parts[1].parse::<i32>() {
            Ok(entry) => entry,
            Err(e) => return Err("Expected an integer for the 'to tube' value".to_string()),
        };
        let quantity = match string_parts[2].parse::<i32>() {
            Ok(entry) => entry,
            Err(e) => return Err("Expected an integer for the 'quantity' value".to_string()),
        };

        if tube_from < 1 || tube_from > game.tubes.len() as i32 {
            return Err("Unexpected 'tube from' number".to_string());
        }
        if tube_to < 1 || tube_to > game.tubes.len() as i32 {
            return Err("Unexpected 'tube to' number".to_string());
        }
        if quantity < 1 || quantity > TUBE_SIZE as i32 {
            return Err("Unexpected 'quantity' number".to_string());
        }

        Ok(MoveInput {
            tube_from,
            tube_to,
            quantity,
        })
    }
}
