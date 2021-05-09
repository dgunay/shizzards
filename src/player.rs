use std::{
    convert::TryFrom,
    fmt::{self, Display},
    io::{Lines, StdinLock},
    iter::FromIterator,
};

use crate::{
    get_input,
    spell::{Reaction, Spell},
    Hand, TryUntilValid,
};

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub hand: Hand,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(PartialEq)]
pub enum Decision {
    Attack,
    Breather,
}

#[derive(Debug)]
pub enum GenericError {
    Oops(String),
}

impl Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenericError::Oops(e) => write!(f, "{}", e),
        }
    }
}

impl TryFrom<&String> for Decision {
    type Error = GenericError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "attack" => Ok(Self::Attack),
            "breather" => Ok(Self::Breather),
            _ => Err(GenericError::Oops(format!(
                "{} is not a valid action",
                value
            ))),
        }
    }
}

impl TryUntilValid<'_, Decision> for Lines<StdinLock<'_>> {
    fn try_until_valid(&mut self) -> Decision {
        loop {
            let next = self.next().expect("msg").expect("msg");
            match Decision::try_from(&next) {
                Ok(d) => return d,
                Err(e) => {
                    println!("{}, try again.", e)
                }
            }
        }
    }
}

impl Player {
    pub fn new(name: String, words: Vec<String>) -> Self {
        Self {
            name,
            hand: Hand::new(words),
        }
    }

    pub fn decision(&self) -> Decision {
        println!("What would {} like to do? [attack, breather]", &self.name);
        get_input()
    }

    pub fn form_spell(&self) -> Spell {
        todo!("form spell");
    }

    pub fn react_to_spell(&self, spell: &Spell) -> Reaction {
        todo!("react")
    }

    pub fn has_words(&self, words: Vec<String>) -> bool {
        words.iter().all(|word| self.hand.words().contains(word))
    }
}
