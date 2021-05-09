use std::{
    borrow::Borrow,
    convert::TryInto,
    io::{stdin, BufRead},
    iter::Filter,
    slice::Iter,
    thread::sleep,
};

use shizzards::{
    game::BaseGame,
    hand::Hand,
    player::{Decision, Player},
    spell::{AttackType, Reaction},
    Game, TryUntilValid,
};

fn main() {
    // Word bank

    // Arg parsing
    let words = vec!["Fire".to_string(), "Blast".to_string()];
    let mut players = vec![
        Player::new("Tim".into(), words.clone()),
        Player::new("Bob".into(), words.clone()),
    ];
    let mut player_iter = players.iter().cycle();

    while players.len() > 1 {
        println!("Advancing a turn");

        // Advance Turn order
        let active_player = player_iter.next().expect("Oops");

        let mut others = players.iter().filter(|p| *p != active_player);

        // Attack someone or call for a breather.
        println!(
            "It is {}'s turn, what will they do? [attack, breather]",
            &active_player.name
        );
        let decision = stdin().lock().lines().try_until_valid();
        match decision {
            Decision::Attack => {
                // Who will you attack?
                let other_names: Vec<String> = others.clone().map(|p| p.name.clone()).collect();
                println!(
                    "{} is attacking! Who will they attack? [{}]",
                    active_player.name,
                    other_names.join(", ")
                );

                let target = try_until_valid_player(others);
                do_fight(
                    active_player,
                    target,
                    third_parties(active_player, target, players.iter()),
                );
            }
            Decision::Breather => {
                println!(
                    "{} is calling for a breather - does anyone object?",
                    active_player.name
                );
                // Does anyone else object?
                if let Some(objecting_player) = others.find(|p| p.decision() == Decision::Attack) {
                    println!("{} objects, prepare to fight!", objecting_player.name);
                    do_fight(
                        objecting_player,
                        active_player,
                        third_parties(objecting_player, active_player, players.iter()),
                    );
                }
            }
        }

        sleep(std::time::Duration::from_secs(60));
    }
}

fn do_fight<'a>(aggressor: &Player, defender: &Player, others: impl Iterator<Item = &'a Player>) {
    println!("What spell will {} cast?", aggressor);
    let mut spell = aggressor.form_spell();

    println!(
        "{} cast '{}'. What kind of spell is this? [direct, hindrance, clever trick]",
        aggressor, &spell
    );

    println!(
        "How will {} react? [deflect, block, counter, frantically dodge]",
        defender
    );
    let neutralized = false;
    while !neutralized {
        match defender.react_to_spell(&spell) {
            Reaction::Deflect { altered_spell } => {
                println!("{} deflects! The spell is now '{}'. What effect did the substitution have? [neutralized, weakened, redirected]", defender, altered_spell);
                spell = altered_spell.clone();
            }
            Reaction::Block { defensive_spell } => {}
            Reaction::Counter { defensive_spell } => {}
            Reaction::FranticallyDodge { words_negated } => {}
        }
    }
}

fn third_parties<'a>(
    a: &'a Player,
    b: &'a Player,
    all_players: impl Iterator<Item = &'a Player>,
) -> impl Iterator<Item = &'a Player> {
    all_players.filter(move |p| *p != a && *p != b)
}

fn try_until_valid_player<'a>(players: impl Iterator<Item = &'a Player>) -> &'a Player {
    let inputs = stdin().lock().lines();
    loop {
        let name = inputs.next().unwrap().unwrap();
        match players.find(|p| p.name == name) {
            Some(p) => return p,
            None => println!("Could not find player '{}', try again.", name),
        }
    }
}

fn try_until_player_has_words(player: &Player) -> Vec<String> {
    let lines = stdin().lock().lines();
    for l in lines {

        let next = stdin().lock().lines().next().expect("msg").expect("msg");
        let words = next.
        match  {
            Ok(spellformation) => return d,
            Err(e) => {
                println!("{}, try again.", e)
            }
        }
    }
}
