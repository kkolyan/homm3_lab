#![allow(non_snake_case)]
#![allow(clippy::collapsible_else_if)]

use std::fmt::{Display, Formatter};
use std::mem;
use rand::Rng;
use crate::combat::UnboundU32::{Inf, Value};
use crate::creature::{Ability, Attr, Creature};

pub struct Stack<'a> {
    pub creature: &'a Creature,
    pub size: u32,
    pub front_unit_health: u32,
}

pub struct FightResult {
    pub army_size: u32,
    pub counts:  [u32; 2],
    pub win_rate: [f32; 2],
}

pub fn find_counter_count(rounds: u32, creature: (u32, &Creature), counter: &Creature, verbose: bool, dry: bool) -> [CounterSearchResult; 2] {
    // println!("find_counter_count ( {}, {} x{}, {}, dry: {} )", rounds, creature.1.name, creature.0, counter.name, dry);
    let mut lower: Option<(u32, f32)> = None;
    let mut upper: Option<(u32, f32)> = None;

    let mut changed = true;

    let initial_estimate_counter_army_size = creature.1.ai_value * creature.0 / counter.ai_value;

    while changed {
        if lower.is_some() && lower.unwrap().0 > initial_estimate_counter_army_size * 100 && lower.unwrap().1 == 1.0 {
            return [
                CounterSearchResult {closest_match_count: Inf, win_ratio: 1.0},
                CounterSearchResult {closest_match_count: Inf, win_ratio: 1.0},
            ]
        }
        changed = false;
        let guess = if upper.is_none() {
            if lower.is_none() {
                initial_estimate_counter_army_size
            } else {
                lower.unwrap().0 * 2 + 1 // +1 to ensure progression in case of zero lower bound
            }
        } else {
            if lower.is_none() {
                upper.unwrap().0 / 2
            } else {
                (lower.unwrap().0 + upper.unwrap().0) / 2
            }
        };
        if guess == 0 && upper.is_some() {
            // println!("guess == 0. bounds: {:?}. {} x{} vs {}. initial guess: {}", lower..upper, creature.1.name, creature.0, counter.name, initial_estimate_counter_army_size);
            return [
                CounterSearchResult {closest_match_count: Value(upper.unwrap().0), win_ratio: upper.unwrap().1},
                CounterSearchResult {closest_match_count: Value(upper.unwrap().0), win_ratio: upper.unwrap().1},
            ]
        }
        let rating = play_match(rounds, creature, (guess, counter), false, dry);
        if verbose {
            println!("  - {} x{} wins {} x{} with {:.01}% rate (bounds: {:?})", creature.1.name, creature.0, counter.name, guess, rating * 100.0, lower..upper);
        }
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
    // when it can't ever win
    if upper.unwrap().0 == 0 && lower.is_none() {
        return [
            CounterSearchResult {closest_match_count: Value(0), win_ratio: upper.unwrap().1},
            CounterSearchResult {closest_match_count: Value(0), win_ratio: upper.unwrap().1},
        ]
    }
    [
        CounterSearchResult { closest_match_count: Value(lower.unwrap().0), win_ratio: lower.unwrap().1 },
        CounterSearchResult { closest_match_count: Value(upper.unwrap().0), win_ratio: upper.unwrap().1 },
    ]
}

#[derive(Debug, Copy, Clone)]
pub struct CounterSearchResult {
    pub closest_match_count: UnboundU32,
    pub win_ratio: f32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnboundU32 {
    Value(u32),
    Inf,
}

impl Display for UnboundU32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value(it) => write!(f, "{}", it),
            Inf => write!(f, "Inf"),
        }
    }
}

pub fn play_match(rounds: u32, a: (u32, &Creature), b: (u32, &Creature), verbose: bool, dry: bool) -> f32 {
    let mut left_win_count = 0;
    let mut right_win_count = 0;
    for _ in 0..rounds {
        let counts = fight(a, b, verbose && rounds == 1);
        if dry {
            if counts.0 < a.0 {
                right_win_count += 1;
            } else {
                left_win_count += 1;
            }
        } else {
            if counts.0 > counts.1 {
                left_win_count += 1;
            }
            if counts.1 > counts.0 {
                right_win_count += 1;
            }
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

    let mut distance = 13;
    if a.creature.attrs_typed.contains(&Attr::DOUBLE_WIDE) {
        distance -= 1;
    }
    if b.creature.attrs_typed.contains(&Attr::DOUBLE_WIDE) {
        distance -= 1;
    }

    while a.size > 0 && b.size > 0 {
        let shoot = a.creature.attrs_typed.contains(&Attr::SHOOTING_ARMY) && distance != 0;

        if shoot || distance <= a.creature.speed {
            attack(a, b, false, !shoot, distance, verbose);

            if a.size > 0 && b.size > 0 {
                if shoot && a.creature.ability_typed.contains(&Ability::ShootsTwice) || a.creature.ability_typed.contains(&Ability::ShootTwice) {
                    // second shot
                    attack(a, b, false, !shoot, distance, verbose);
                }

                if a.size > 0 && b.size > 0 {
                    // retaliate
                    retaliate_if_valid(a, b, verbose, shoot);
                    if a.size > 0 && b.size > 0 {
                        // second strike
                        if !shoot && a.creature.ability_typed.contains(&Ability::StrikesTwice) {
                            attack(a, b, false, true, 0, verbose);
                            // retaliate once again, if Griffon
                            if b.creature.ability_typed.contains(&Ability::RetaliatesTwice) ||b.creature.ability_typed.contains(&Ability::UnlimitedRetaliations) {
                                retaliate_if_valid(a, b, verbose, shoot);
                            }
                        }
                    }
                }
            }
        } else {
            if verbose {
                println!("{} moves towards {}", a.creature.name, b.creature.name);
            }
            distance = distance.saturating_sub(a.creature.speed);
        }
        mem::swap(&mut a, &mut b);
    }
}

fn retaliate_if_valid(attacker: &mut Stack, defender: &mut Stack, verbose: bool, shoot: bool) {
    if !shoot && !attacker.creature.ability_typed.contains(&Ability::EnemiesCannotRetaliate) && !attacker.creature.ability_typed.contains(&Ability::NoEnemyRetaliation) {
        attack(defender, attacker, true, true, 0, verbose);
    }
}

fn attack(attacker: &mut Stack, defender: &mut Stack, retaliation: bool, melee: bool, distance: u32, verbose: bool) {
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
    let R5 = if melee {
        if attacker.creature.attrs_typed.contains(&Attr::SHOOTING_ARMY) && !attacker.creature.ability_typed.contains(&Ability::NoMeleePenalty) {
            0.5
        } else {
            0.0
        }
    } else {
        if distance >= 10 { 0.5 } else { 0.0 }
    }; // range or melee penalty
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