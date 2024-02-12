#![allow(unused)]

use std::io;

use crate::{game::Game, repl::Repl};

pub mod game;
pub mod repl;
pub mod tube;

pub const TUBE_SIZE: usize = 4;

fn main() {
    println!("Welcome to Water Sorter Solver!");
    println!("Starting a new game...");
    let mut game = Game::default();
    let mut repl = Repl::new(io::stdin(), io::stdout(), game);
    repl.start();
    repl.play();
}
