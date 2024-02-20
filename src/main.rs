use std::io;

use crate::{game::Game, repl::Repl};

pub mod game;
pub mod repl;
pub mod solver;
pub mod tube;

pub const TUBE_SIZE: usize = 4;

fn main() {
    println!("Welcome to Water Sorter Solver!");
    println!("Starting a new game...");
    let game = Game::default();
    let mut repl = Repl::new(io::stdin(), io::stdout(), game);
    let mut initialized = false;
    while !initialized {
        initialized = repl.start();
    }
    repl.play();
}
