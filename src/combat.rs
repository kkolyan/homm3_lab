#![allow(non_snake_case)]
#![allow(clippy::collapsible_else_if)]

use std::fmt::{Display, Formatter};
use std::mem;
use std::str::FromStr;
use rand::{random, Rng};
use crate::combat::UnboundU32::{Inf, Value};
use crate::creature::Creature;
use crate::creature::Feature::{Ages, Curses, DeathBlow, DeathStare, DoubleWide, EnemiesCannotRetaliate, FireShield, ImmuneToFire, ImmuneToJoustingBonus, ImmuneToMagic, ImmuneToMagic1to3, ImmuneToMagic1to4, JoustingBonus, NoMeleePenalty, Poisonous, RetaliatesTwice, Shoots, ShootsTwice, StrikesTwice, TargetEnemysDefenseIsReduced40Percent, TargetEnemysDefenseIsReduced80Percent, Undead, UnlimitedRetaliations, Unliving};

pub struct Stack<'a> {
    pub creature: &'a Creature,
    pub size: u32,
    pub front_unit_health: u32,
    pub poison_count: u32,
    pub age_rounds_remaining: u32,
    pub curse_rounds_remaining: u32,
    pub poison_rounds_remaining: u32,
}

impl<'b> Stack<'b> {

    fn create(size: u32, creature: &'b Creature) -> Stack<'b> {
        Self {
            creature,
            size,
            front_unit_health: creature.health,
            poison_count: 0,
            age_rounds_remaining: 0,
            curse_rounds_remaining: 0,
            poison_rounds_remaining: 0,
        }
    }
    fn calculate_max_health(&self) -> u32 {
        let age_factor: f32 = if self.age_rounds_remaining > 0 { 0.5 } else { 0.0 };
        let poison_factor = self.poison_count as f32 * 0.1;
        (self.creature.health as f32 * (1.0 - age_factor) * (1.0 - poison_factor)).ceil() as u32
    }
}

pub struct FightResult {
    pub army_size: u32,
    pub counts: [u32; 2],
    pub win_rate: [f32; 2],
}

pub fn find_counter_count(rounds: u32, creature: (u32, &Creature), counter: &Creature, verbose: bool, clean: bool) -> [CounterSearchResult; 2] {
    // println!("find_counter_count ( {}, {} x{}, {}, clean: {} )", rounds, creature.1.name, creature.0, counter.name, clean);
    let mut lower: Option<(u32, f32)> = None;
    let mut upper: Option<(u32, f32)> = None;

    let mut changed = true;

    let initial_estimate_counter_army_size = creature.1.ai_value * creature.0 / counter.ai_value + 1;

    while changed {
        if lower.is_some() && lower.unwrap().0 > initial_estimate_counter_army_size * 100 && lower.unwrap().1 == 1.0 {
            assert!(initial_estimate_counter_army_size > 0);
            return [
                CounterSearchResult { closest_match_count: Inf, win_ratio: 1.0 },
                CounterSearchResult { closest_match_count: Inf, win_ratio: 1.0 },
            ];
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
                CounterSearchResult { closest_match_count: Value(upper.unwrap().0), win_ratio: upper.unwrap().1 },
                CounterSearchResult { closest_match_count: Value(upper.unwrap().0), win_ratio: upper.unwrap().1 },
            ];
        }
        let rating = play_match(rounds, creature, (guess, counter), false, clean);
        if verbose {
            println!("  - {} x{} wins {} x{} with {:.01}% rate (bounds: {:?})", creature.1.name, creature.0, counter.name, guess, rating * 100.0, lower..upper);
        }
        let threshold = if clean { 1.0 } else { 0.5 };
        if rating >= threshold {
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
            CounterSearchResult { closest_match_count: Value(0), win_ratio: upper.unwrap().1 },
            CounterSearchResult { closest_match_count: Value(0), win_ratio: upper.unwrap().1 },
        ];
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

impl FromStr for UnboundU32 {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Inf" {
            return Ok(Inf);
        }
        Ok(Value(s.parse()?))
    }
}

pub fn play_match(rounds: u32, a: (u32, &Creature), b: (u32, &Creature), verbose: bool, clean: bool) -> f32 {
    let mut left_win_count = 0;
    let mut right_win_count = 0;
    for _ in 0..rounds {
        let counts = fight(a, b, verbose && rounds == 1);
        if clean {
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
        println!("{}\n  VS\n{}\n", a.1.combat_info(), b.1.combat_info());
    }
    let mut a = Stack::create(a.0, a.1);
    let mut b = Stack::create(b.0, b.1);

    fight_1(&mut a, &mut b, verbose);

    (a.size, b.size)
}

fn fight_1<'a, 'b>(mut a: &'a mut Stack<'b>, mut b: &'a mut Stack<'b>, verbose: bool) {
    if a.creature.speed < b.creature.speed {
        mem::swap(&mut a, &mut b);
    }

    let mut distance = 13;
    if a.creature.has_feature(DoubleWide) {
        distance -= 1;
    }
    if b.creature.has_feature(DoubleWide) {
        distance -= 1;
    }

    // to avoid situation when faster creature almost always get damaged first
    // TODO implement a trick when faster creature waits to effectively have two attacks, which in case of non-retaliable attack may change a score
    if !a.creature.has_feature(Shoots) && !b.creature.has_feature(Shoots) {
        distance = 0;
    }

    let mut semi_turns = 0;

    while a.size > 0 && b.size > 0 {
        let shoot = a.creature.has_feature(Shoots) && distance != 0;

        if shoot || distance <= a.creature.speed {
            let running_distance =  if !shoot {
                let running_distance = if semi_turns < 2 {
                    // emulate that Champion will have less bonus if enemy is fast enough to almost get close
                    11u32.saturating_sub(if b.creature.has_feature(Shoots) {0} else {b.creature.speed})
                } else {
                    0
                };
                let running_distance = running_distance.min(a.creature.speed);
                // emulate running around an enemy every turn
                running_distance.max(3)
            } else {
                0
            };
            if !shoot {
                distance = 0;
            }
            attack(a, b, false, !shoot, distance, verbose, running_distance);

            if a.size > 0 && b.size > 0 {
                if shoot && a.creature.has_feature(ShootsTwice) {
                    // second shot
                    attack(a, b, false, !shoot, distance, verbose, 0);
                }

                if a.size > 0 && b.size > 0 {
                    // retaliate
                    retaliate_if_valid(a, b, verbose, shoot);
                    if a.size > 0 && b.size > 0 {
                        // second strike
                        if !shoot && a.creature.has_feature(StrikesTwice) {
                            attack(a, b, false, true, 0, verbose, 0);
                            // retaliate once again, if Griffon
                            if b.creature.has_feature(RetaliatesTwice) || b.creature.has_feature(UnlimitedRetaliations) {
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
        semi_turns += 1;
        if semi_turns % 2 == 0 {
            if verbose {
                println!("Turn {} finished", 1 + semi_turns / 2)
            }
            tick_effects(a, verbose);
            tick_effects(b, verbose);
        }
    }
}

fn retaliate_if_valid(attacker: &mut Stack, defender: &mut Stack, verbose: bool, shoot: bool) {
    if !shoot && !attacker.creature.has_feature(EnemiesCannotRetaliate) {
        attack(defender, attacker, true, true, 0, verbose, 0);
    }
}

fn attack(attacker: &mut Stack, defender: &mut Stack, retaliation: bool, melee: bool, distance: u32, verbose: bool, running_distance: u32) {
    let DMGb: u32 = if attacker.curse_rounds_remaining > 0 {
        attacker.creature.damage_low * attacker.size
    } else {
        if attacker.size <= 10 {
            (0..attacker.size).map(|_| rand::thread_rng().gen_range(attacker.creature.damage_low..attacker.creature.damage_high + 1)).sum()
        } else {
            let rand_10: u32 = (0..10).map(|_| rand::thread_rng().gen_range(attacker.creature.damage_low..attacker.creature.damage_high + 1)).sum();
            rand_10 * attacker.size / 10
        }
    };

    let A = attacker.creature.attack as f32;
    let mut D = defender.creature.defence as f32;
    if attacker.creature.has_feature(TargetEnemysDefenseIsReduced80Percent) {
        D *= 0.2;
    } else if attacker.creature.has_feature(TargetEnemysDefenseIsReduced40Percent) {
        D *= 0.6;
    }
    let I1 = if A >= D {
        0.05 * (A - D)
    } else { 0f32 };

    let I2 = 0f32;// archery, offence skills
    let I3 = 0f32;// offence speciality, Adela's bless
    let I4 = 0f32; // luck
    let mut I5 = 0f32; // cannon tripple damage, death blow, ballista or cannon double damage, elementals hate (hidden in game), hate, cavalry bonus.
    if attacker.creature.hates.contains(&defender.creature.id) {
        I5 = 0.5;
    }
    let death_blow = attacker.creature.has_feature(DeathBlow) && random::<f32>() < 0.2;
    if death_blow {
        I5 = 1.0;
    }
    if attacker.creature.has_feature(JoustingBonus) && !defender.creature.has_feature(ImmuneToJoustingBonus) {
        I5 = 0.05 * running_distance as f32;
    }
    let R1 = if D >= A { 0.025 * (D - A) } else { 0f32 };
    let R2 = 0f32;// armorer skill
    let R3 = 0f32;// armorer speciality
    let R4 = 0f32; // shield, air shield
    let R5 = if melee {
        if attacker.creature.has_feature(Shoots) && !attacker.creature.has_feature(NoMeleePenalty) {
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
        let size_before = defender.size;
        apply_damage(defender, DMGf, &mut kills);
        if verbose {
            println!(
                "{} (x{}) {} {} (x{}) for {} hp. {} killed ({} left, top hp: {}).{}",
                attacker.creature.name,
                attacker.size,
                if retaliation { "retaliates" } else { "attacks" },
                defender.creature.name,
                size_before,
                DMGf,
                kills,
                defender.size,
                defender.front_unit_health,
                if death_blow {" Death Blow triggered."} else {""}
            );
        }
    }
    apply_death_stare(attacker, defender, verbose);
    if defender.creature.has_feature(FireShield) && !attacker.creature.has_feature(ImmuneToFire) {
        let mut kills = 0;
        let FireShieldDamage = 0.2 * DMGb as f32 * (1.0 + I1 + I2 + I3 + I4 + I5);
        let size_before = attacker.size;
        apply_damage(attacker, FireShieldDamage, &mut kills);
        if verbose {
            println!(
                "{} (x{})'s fire shield hits {} (x{}) for {} hp. {} killed ({} left, top hp: {}).",
                defender.creature.name,
                size_before,
                attacker.creature.name,
                attacker.size,
                FireShieldDamage,
                kills,
                attacker.size,
                attacker.front_unit_health,
            );
        }
        apply_death_stare(attacker, defender, verbose);
    }
    if attacker.creature.has_feature(Poisonous) && random::<f32>() < 0.3 {
        defender.poison_rounds_remaining = 3;
        tick_poison(defender, verbose);
    }
    if attacker.creature.has_feature(Ages) && !defender.creature.has_any_feature([Undead, Unliving, ImmuneToMagic]) && random::<f32>() < 0.2 {
        let wounds = defender.calculate_max_health() - defender.front_unit_health;
        let already_aged = defender.age_rounds_remaining > 0;
        defender.age_rounds_remaining = 3;
        if !already_aged {
            defender.front_unit_health = (defender.calculate_max_health() as i32 - wounds as i32).max(1_i32) as u32;
            if verbose {
                println!("{} aged and its health reduced to {}", defender.creature.name, defender.calculate_max_health());
            }
        }
    }
    if attacker.creature.has_feature(Curses) && !defender.creature.has_any_feature([ImmuneToMagic, ImmuneToMagic1to3, ImmuneToMagic1to4, ImmuneToFire, Undead]) && random::<f32>() < 0.25 {
        let already_cursed = defender.curse_rounds_remaining > 0;
        defender.curse_rounds_remaining = 3;
        if !already_cursed && verbose {
            println!("{} cursed", defender.creature.name);
        }
    }
}

fn tick_effects(target: &mut Stack, verbose: bool) {

    tick_poison(target, verbose);
    tick_aging(target, verbose);
    tick_curse(target, verbose);
}

fn tick_curse(target: &mut Stack, verbose: bool) {
    if target.curse_rounds_remaining > 0 {
        target.curse_rounds_remaining -= 1;
        if verbose && target.curse_rounds_remaining == 0 {
            println!("{} curse worn off", target.creature.name);
        }
    }
}

fn tick_poison(target: &mut Stack, verbose: bool) {
    if target.poison_rounds_remaining > 0 {
        if target.poison_count < 5 {
            let wounds = target.calculate_max_health() - target.front_unit_health;

            target.poison_count += 1;
            target.front_unit_health = (target.calculate_max_health().saturating_sub(wounds)).max(1);
            if verbose {
                println!("{} poisoned and its max health reduced to {}", target.creature.name, target.calculate_max_health());
            }
        }
        target.poison_rounds_remaining -= 1;
    }
}

fn tick_aging(target: &mut Stack, verbose: bool) {
    if target.age_rounds_remaining > 0 {
        target.age_rounds_remaining -= 1;

        if target.age_rounds_remaining == 0 && verbose {
            println!("{} aging worn of and health restored to {}", target.creature.name, target.calculate_max_health());
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
            defender.front_unit_health = defender.calculate_max_health();
            *kills += 1;
        }
    }
}

fn apply_death_stare(attacker: &Stack, defender: &mut Stack, verbose: bool) {
    let mut kills = 0;
    if attacker.creature.has_feature(DeathStare)
        && !defender.creature.has_feature(Undead)
        && !defender.creature.has_feature(Unliving)
    {
        let stare_kills = (0..attacker.size).filter(|_| random::<f32>() <= 0.1).count() as u32;
        let stare_kills = stare_kills.min((attacker.size as f32 / 10_f32).ceil() as u32);

        let stare_kills = stare_kills.min(defender.size);

        kills += stare_kills;
        defender.size -= stare_kills;
        if stare_kills > 0 {
            defender.front_unit_health = defender.calculate_max_health();
        }
    }
    if kills > 0 && verbose {
        println!("{} {} were death-stared. ({} left)", kills, defender.creature.name_plural, defender.size)
    }
}