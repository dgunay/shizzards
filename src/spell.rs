use std::{
    convert::TryFrom,
    fmt::{write, Display},
    io::{Lines, StdinLock},
    u32,
};

use crate::player::GenericError;

#[derive(Debug, Clone)]
pub struct Spell {
    pub words: Vec<String>,
    pub spelltype: SpellType,
}

#[derive(Debug, Clone)]
pub enum SpellType {
    Attack(AttackType),
    Defense,
}

impl Spell {}

impl Display for Spell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.words.join(", "))
    }
}

pub enum Reaction {
    Deflect { altered_spell: Spell },
    Block { defensive_spell: Spell },
    Counter { defensive_spell: Spell },
    FranticallyDodge { words_negated: u32 },
}

#[derive(Debug, Clone)]
pub enum AttackType {
    Direct,
    Hindrance,
    CleverTrick,
}

impl TryFrom<String> for AttackType {
    type Error = GenericError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "direct" => Ok(Self::Direct),
            "hindrance" => Ok(Self::Hindrance),
            "clever trick" => Ok(Self::CleverTrick),
            _ => Err(GenericError::Oops(format!(
                "{} is not a valid type of attack",
                value
            ))),
        }
    }
}

pub enum DefenseType {}

pub enum Nature {
    Attack(AttackType),
    Defense(DefenseType),
}
