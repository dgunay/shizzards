use std::{iter::Cycle, slice::Iter};

use crate::{player::Player, Game};

pub struct BaseGame {
    players: Vec<Player>,
}

impl BaseGame {
    pub fn new(players: Vec<Player>) -> Self {
        Self { players }
    }
}

impl<'a> Game<'a, Iter<'a, Player>> for BaseGame {
    fn finished(&self) -> bool {
        todo!()
    }

    fn player_iter(&'a mut self) -> Cycle<Iter<'a, Player>> {
        self.players.iter().cycle()
    }
}
