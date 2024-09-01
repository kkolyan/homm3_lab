use std::collections::HashSet;
use std::mem;
use rand::Rng;
use crate::creature::{Ability, Creature};

pub struct Stack<'a> {
    pub creature: &'a Creature,
    pub size: u32,
    pub front_unit_health: u32,
}

pub fn find_counter_count(rounds: u32, creature: (u32, &Creature), counter: &Creature) -> Vec<CounterSearchResult> {
    let mut lower: Option<(u32, f32)> = None;
    let mut upper: Option<(u32, f32)> = None;

    let mut changed = true;

    while lower.is_none() || upper.is_none() || changed {
        changed = false;
        let guess = if upper.is_none() {
            if lower.is_none() {
                creature.1.ai_value * creature.0 / counter.ai_value
            } else {
                lower.unwrap().0 * 2
            }
        } else {
            if lower.is_none() {
                upper.unwrap().0 / 2
            } else {
                (lower.unwrap().0 + upper.unwrap().0) / 2
            }
        };
        let rating = play_match(rounds, creature, (guess, counter), false);
        // println!("  - {} x{} wins {} x{} with {:.01} rate", creature.1.name, creature.0, counter.name, guess, rating);
        if rating > 0.5 {
            if lower.is_none() || lower.unwrap().0 != guess {
                changed = true;
            }
            lower = Some((guess, rating));
        } else {
            if upper.is_none() || upper.unwrap().0 != guess {
                changed = true;
            }
            upper = Some((guess, rating));
        }
    }
    //
    // if (lower.unwrap().1 - 0.5).abs() < (upper.unwrap().1 - 0.5).abs() {
    //     return vec![CounterSearchResult { closest_match_count: lower.unwrap().0, win_ratio: lower.unwrap().1 }];
    // }
    // vec![
    //     CounterSearchResult { closest_match_count: upper.unwrap().0, win_ratio: upper.unwrap().1 },
    // ]
    if lower.unwrap().0 == upper.unwrap().0 {
        return vec![CounterSearchResult { closest_match_count: lower.unwrap().0, win_ratio: lower.unwrap().1 }];
    }
    vec![
        CounterSearchResult { closest_match_count: lower.unwrap().0, win_ratio: lower.unwrap().1 },
        CounterSearchResult { closest_match_count: upper.unwrap().0, win_ratio: upper.unwrap().1 },
    ]
}

#[derive(Debug)]
pub struct CounterSearchResult {
    pub closest_match_count: u32,
    pub win_ratio: f32,
}

pub fn play_match(rounds: u32, a: (u32, &Creature), b: (u32, &Creature), verbose: bool) -> f32 {
    let mut left_win_count = 0;
    let mut right_win_count = 0;
    for _ in 0..rounds {
        let counts = fight(a, b, verbose && rounds == 1);
        if counts.0 > counts.1 {
            left_win_count += 1;
        }
        if counts.1 > counts.0 {
            right_win_count += 1;
        }
    }

    let rating = left_win_count as f32 / (left_win_count + right_win_count) as f32;

    if verbose {
        println!("{} x{} vs {} x{}: {:.01}%", a.1.name, a.0, b.1.name, b.0, 100f32 * rating);
    }
    rating
}

pub fn fight(a: (u32, &Creature), b: (u32, &Creature), verbose: bool) -> (u32, u32) {
    if verbose {
        println!("{:?}\n  VS\n {:?}\n", a.1, b.1);
    }
    let mut a = Stack {
        creature: a.1,
        size: a.0,
        front_unit_health: a.1.health,
    };
    let mut b = Stack {
        creature: b.1,
        size: b.0,
        front_unit_health: b.1.health,
    };

    fight_1(&mut a, &mut b, verbose);

    (a.size, b.size)
}

fn fight_1<'a, 'b>(mut a: &'a mut Stack<'b>, mut b: &'a mut Stack<'b>, verbose: bool) {
    if a.creature.speed < b.creature.speed {
        mem::swap(&mut a, &mut b);
    }

    while a.size > 0 && b.size > 0 {
        attack(a, b, false, verbose);
        if b.size > 0 && !a.creature.ability_typed.contains(&Ability::EnemiesCannotRetaliate) && !a.creature.ability_typed.contains(&Ability::NoEnemyRetaliation) {
            attack(b, a, true, verbose);
        }
        mem::swap(&mut a, &mut b);
    }
}

fn attack(attacker: &mut Stack, defender: &mut Stack, retaliation: bool, verbose: bool) {
    let DMGb: u32 = if attacker.size <= 10 {
        (0..attacker.size).map(|_| rand::thread_rng().gen_range(attacker.creature.damage_low..attacker.creature.damage_high + 1)).sum()
    } else {
        let rand_10: u32 = (0..10).map(|_| rand::thread_rng().gen_range(attacker.creature.damage_low..attacker.creature.damage_high + 1)).sum();
        rand_10 * attacker.size / 10
    };

    let A = attacker.creature.attack;
    let D = defender.creature.defence;
    let I1 = if A >= D {
        0.05 * (A - D) as f32
    } else { 0f32 };

    let I2 = 0f32;// archery, offence skills
    let I3 = 0f32;// offence speciality, Adela's blell
    let I4 = 0f32; // luck
    let I5 = 0f32; // cannon tripple damage, death blow, ballista or cannon double damage, elementals hate (hidden in game), hate, cavalry bonus.
    let R1 = if D >= A { 0.025 * (D - A) as f32 } else { 0f32 };
    let R2 = 0f32;// armorer skill
    let R3 = 0f32;// armorer speciality
    let R4 = 0f32; // shield, air shield
    let R5 = 0f32; // range or melee penalty
    let R6 = 0f32; // obstacle penalty
    let R7 = 0f32; // blind retaliation, forgetfulness
    let R8 = 0f32; // elemental attack immunity (hidden), petrificated target, paralyzed retaliation
    let R9 = 0f32; // unluck (HOTA only)

    {
        let DMGf = DMGb as f32 * (1.0 + I1 + I2 + I3 + I4 + I5) * (1.0 - R1) * (1.0 - R2 - R3) * (1.0 - R4) * (1.0 - R5) * (1.0 - R6) * (1.0 - R7) * (1.0 - R8) * (1.0 - R9);

        let mut kills = 0;
        apply_damage(defender, DMGf, &mut kills);
        if verbose {
            println!("{} (x{}) {} {} (x{}) for {} hp. {} killed ({} left).", attacker.creature.name, attacker.size, if retaliation { "retaliates" } else { "attacks" }, defender.creature.name, defender.size, DMGf, kills, defender.size);
        }
    }
    if defender.creature.ability_typed.contains(&Ability::FireShield) {
        let mut kills = 0;
        let FireShieldDamage = 0.2 * DMGb as f32 * (1.0 + I1 + I2 + I3 + I4 + I5);
        apply_damage(attacker, FireShieldDamage, &mut kills);
        if verbose {
            println!("{} (x{})'s fire shield hits {} (x{}) for {} hp. {} killed ({} left).", defender.creature.name, attacker.size, attacker.creature.name, attacker.size, FireShieldDamage, kills, attacker.size);
        }
    }
}

fn apply_damage(defender: &mut Stack, DMGf: f32, kills: &mut u32) {
    let mut d_rem = DMGf as u32;
    while d_rem > 0 && defender.size > 0 {
        if defender.front_unit_health > d_rem {
            defender.front_unit_health -= d_rem;
            d_rem = 0;
        } else {
            d_rem -= defender.front_unit_health;
            defender.size -= 1;
            defender.front_unit_health = defender.creature.health;
            *kills += 1;
        }
    }
}