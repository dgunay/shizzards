use std::{
    io::{stdin, BufRead},
    thread::sleep,
};

use shizzards::{
    get_input,
    player::{Decision, Player},
    spell::Reaction,
};

fn main() {
    // Word bank

    // Arg parsing
    let words = vec!["Fire".to_string(), "Blast".to_string()];
    let mut players = vec![
        Player::new("Tim".into(), words.clone()),
        Player::new("Bob".into(), words.clone()),
    ];
    let mut player_iter = players.iter_mut().cycle();

    while players.len() > 1 {
        println!("Advancing a turn");

        // Advance Turn order
        let mut active_player = player_iter.next().expect("Oops");

        let mut others = players.iter().filter(|p| *p != active_player);

        // Attack someone or call for a breather.
        println!(
            "It is {}'s turn, what will they do? [attack, breather]",
            &active_player.name
        );
        let decision = get_input();
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
                let tgt_clone = target.clone();
                let others = third_parties(active_player, &tgt_clone, players.iter());
                do_fight(active_player, target, others);
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
                        &mut active_player,
                        third_parties(objecting_player, active_player, players.iter()),
                    );
                }
            }
        }

        sleep(std::time::Duration::from_secs(60));
    }
}

fn do_fight<'a>(
    aggressor: &Player,
    defender: &mut Player,
    others: impl Iterator<Item = &'a Player>,
) {
    println!("What spell will {} cast?", aggressor);
    let words = stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(" ")
        .map(str::to_string)
        .collect();

    let ask_for_attack_type = |words| {
        println!(
            "{} is casting {:?} on {}. What kind of spell is this? [direct, hindrance, clever trick]",
            aggressor, words, defender,
        );
        get_input()
    };
    let mut spell = aggressor
        .form_attack_spell(&words, ask_for_attack_type)
        .expect("TODO: deal with bad spells");

    let mut neutralized = false;
    while !neutralized {
        println!("Spell is inbound with words '{:?}'.", spell);
        println!(
            "How will {} react? [deflect, block, counter, frantically dodge]",
            defender
        );
        let reaction = match defender.react_to_spell(&spell) {
            Reaction::Deflect { altered_spell } => {
                println!("{} deflects! The spell is now '{}'. What effect did the substitution have? [neutralized, weakened, redirected]", defender, altered_spell);
                spell = altered_spell.clone();
                todo!("Determine effect");
                true
            }
            Reaction::Block { defensive_spell } => {
                // neutralize as many words in the original spell
                for _ in defensive_spell.words {
                    spell.words.pop();
                }

                spell.words.is_empty()
            }
            Reaction::Counter { defensive_spell } => {
                println!(
                    "{} tries to perfectly counter with '{}'! Did it work? [yes, no]",
                    defender, defensive_spell
                );

                // If the other wizards are cool with it, the defender is safe.
                // otherwise, return the words to the defender's hand.
                let response = stdin().lock().lines().next().unwrap().unwrap();
                match response.as_str() {
                    "yes" => true,
                    _ => {
                        defensive_spell
                            .words
                            .iter()
                            .for_each(|w| defender.hand.give_word(w.clone()));
                        false
                    }
                }
            }
            Reaction::FranticallyDodge { words_negated } => {
                // Give up two cards for every word negated.
                // TODO: allow the player to choose words.
                loop {
                    println!("Pick words from your hand. Every two will negate one word from the inbound spell.");
                    let words_to_discard = try_until_player_has_words(defender);
                    if words_to_discard.len() % 2 == 0 {
                        words_to_discard
                            .iter()
                            .for_each(|w| drop(defender.hand.take_word(w).unwrap()));

                        for i in 0..words_to_discard.len() / 2 {
                            spell.words.pop();
                        }
                        break;
                    } else {
                        println!("Must be an even number of words!");
                    }
                }

                spell.words.is_empty()
            }
        };

        if neutralized {
            println!("Spell is neutralized - {} is safe!", defender);
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

fn try_until_valid_player<'a>(mut players: impl Iterator<Item = &'a Player>) -> &'a mut Player {
    loop {
        let name = stdin().lock().lines().next().unwrap().unwrap();
        match players.find(|p| p.name == name) {
            Some(p) => return &mut p,
            None => println!("Could not find player '{}', try again.", name),
        }
    }
}

fn try_until_player_has_words(player: &Player) -> Vec<String> {
    loop {
        let line = stdin().lock().lines().next().unwrap().unwrap();
        let words = line.split(" ").map(str::to_string).collect();
        match player.has_words(&words) {
            true => return words,
            false => {
                println!(
                    "Player {} does not have all of words '{:?}', try again.",
                    player, words
                );
            }
        }
    }
}
