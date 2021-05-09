pub mod game;
pub mod hand;
pub mod player;
pub mod spell;

use std::io::{stdin, BufRead};
use std::{convert::TryFrom, iter::Cycle};

use hand::Hand;
use player::Player;

/// The list of usable words in the game
pub trait Dictionary {
    fn get_random_word(&self) -> String;
}

pub trait Game<'a, I: Iterator<Item = &'a Player>> {
    fn finished(&'a self) -> bool;
    fn player_iter(&'a mut self) -> Cycle<I>;
}

pub trait TryUntilValid<'a, Enum: TryFrom<&'a String>> {
    fn try_until_valid(&mut self) -> Enum;
}

fn get_input<T>() -> T {
    stdin().lock().lines().try_until_valid()
}
