use std::{
    fmt::{write, Display},
    u32,
};

#[derive(Debug, Clone)]
pub struct Spell {
    pub words: Vec<String>,
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

pub enum AttackType {
    Direct,
    Hindrance,
    CleverTrick,
}

pub enum DefenseType {}

pub enum Nature {
    Attack(AttackType),
    Defense(DefenseType),
}
