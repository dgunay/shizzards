pub mod game;
pub mod hand;
pub mod player;
pub mod spell;

use std::io::{stdin, BufRead, StdinLock};
use std::{convert::TryFrom, iter::Cycle};

use hand::Hand;
use player::{GenericError, Player};

/// The list of usable words in the game
pub trait Dictionary {
    fn get_random_word(&self) -> String;
}

pub trait Game<'a, I: Iterator<Item = &'a Player>> {
    fn finished(&'a self) -> bool;
    fn player_iter(&'a mut self) -> Cycle<I>;
}

pub fn get_input<T>() -> T
where
    T: TryFrom<String>,
    <T as TryFrom<String>>::Error: std::fmt::Display,
{
    // Read stdin until it is tryfrommable
    let stdin = stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);
    for l in lines {
        let s = l.clone();
        match T::try_from(s) {
            Ok(ok) => return ok,
            Err(e) => (),
        }
    }

    panic!("should never happen")
}

// impl TryFrom<&String> for bool {
//     type Error = GenericError;

//     fn try_from(value: &String) -> Result<Self, Self::Error> {
//         match value.as_str().to_lowercase() {
//             "yes" => Ok(true),
//             "no" => Ok(false),
//             "true" => Ok(true),
//             "false" => Ok(false),
//         }
//     }
// }
