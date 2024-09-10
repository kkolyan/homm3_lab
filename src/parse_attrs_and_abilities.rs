#![allow(clippy::needless_if)]

use std::collections::HashMap;
use crate::creature::{Creature, Feature};
use crate::creature::Ability::*;
use crate::creature::Attr::*;
use crate::creature::Feature::{DeathStare, DoubleWide, EnemiesCannotRetaliate, FireShield, Hates, NoMeleePenalty, RetaliatesTwice, Shoots, ShootsTwice, StrikesTwice, TargetEnemysDefenseIsReduced40Percent, Undead, UnlimitedRetaliations, Unliving};

pub fn parse_attrs_and_abilities(creature: &mut Creature) {
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Target enemy's defense is reduced 80%")) { creature.features.push(Feature::TargetEnemysDefenseIsReduced80Percent); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Vulnerable to ice")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Mind & fire immunity")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("cold vulnerability")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Shoot twice")) { creature.features.push(ShootsTwice); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("-1 enemy luck")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Merry")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Shoots twice")) { creature.features.push(ShootsTwice); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Disease")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Casts Protect from Fire")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Weakens enemies")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Strikes twice")) { creature.features.push(StrikesTwice); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("+1(+2) morale")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("20% magic resistance")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Immune to fire")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Mind spell immunity")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Increase Hero's Knowledge by 1-3 per week")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Aura of magic resistance")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("-1 to enemy morale")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Death stare")) { creature.features.push(DeathStare)  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Increase Hero's Defense Skill by 1-3 per week")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Attacks all adjacent enemies")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Protected against air spells")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("No walls or range penalty")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Spits acid")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Hero spells cost less")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Drains enemy mana")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Dispels beneficial spells")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Retaliates twice")) { creature.features.push(RetaliatesTwice); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Damage from spells reduced 85%")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Fire spell Immunity")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Lycanthropy (x2 if full moon)")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Hates Black Dragons")) { creature.features.push(Hates(vec!["Black Dragon"])); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Spellcaster: Bloodlust")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Ignores defence")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Immune to ice")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Takes Soul")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("40% magic resistance")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Lightning and firestorm vulnerability")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Ages enemy")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Protected by Magic Mirror")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Can gift")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("3-headed attack")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Ranged attacker")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Good morale")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Extended Death stare")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("20% Block")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Fear")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Petrifying attack")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Attacks all adjacent enemies w/o retaliation")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Fireball attack")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("No Range or Barrier penalties")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("-1(-2) enemy luck")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("No fear")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Spell immunity")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Blinding attack")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Paralyzing venom")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Immune to spell levels 1-3")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Immune to spell levels 1-4")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Summon demons from")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Attacks siege walls")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Group spell casters")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Curse")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("+1 Gem daily")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Heals troops")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Berserk if full moon: days 14-16")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Enemies cannot retaliate")) { creature.features.push(EnemiesCannotRetaliate); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Unlimited retaliations")) { creature.features.push(UnlimitedRetaliations); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Death cloud attack")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("No melee penalty")) { creature.features.push(NoMeleePenalty); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Thunderclap")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Resurrects allies")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Magic damper")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Fearsome")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Spying")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Damage from spells reduced 50%")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Offensive spell caster")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Rebirth")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Summon guards")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Hates Efreet")) { creature.features.push(Hates(vec!["Efreeti", "Efreet Sultan"])); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Strike and return")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Ice Bolt attack")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Jousting bonus")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Vulnerable to fire")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Protected against earth spells")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Can blind")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Meteor shower vulnerability")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Crystal generation")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("No Enemy retaliation")) { creature.features.push(EnemiesCannotRetaliate); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Protected against water spells")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Damage from spells reduced 95%")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Sandwalkers")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Resurrects twice")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("+1(+2) luck")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Increase Hero's Spell Power by 1-3 per week")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Positive luck")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Immune to Fire Magic")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Protected against fire spells")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Unlimited ammunition")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Increases Hero's Attack Skill by 1-3 per week")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Damage from spells reduced 75%")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Regenerating")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Sucks blood")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("+1 morale")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Fire vulnerability")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Immune to Blinding")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Poisonous")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Immune to Champion charge bonus")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Answers twice")) { creature.features.push(RetaliatesTwice); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Can regenerate")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Ranged spell caster")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Air shield")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("a dead ally")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Immune to all spells")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Stone gaze")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Ice immunity")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Attract dead souls")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Spellcaster: Random benefit")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Death Blow attack")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Fire Wall")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Magic channel")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Casts protection from Earth")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Breath attack")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Ignores obstacles")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Fire Shield")) { creature.features.push(FireShield); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Binds enemies in place")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Curses enemies")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Undead")) { creature.features.push(Undead) }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Hates Devils")) { creature.features.push(Hates(vec!["Devil", "Arch Devil"])); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Attack ages enemies")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Gets back")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Slayer")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Rebirth always")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Darkness")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("20% Magic Resist")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Hates Genies")) { creature.features.push(Hates(vec!["Genie", "Master Genie"])); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Lightning strike")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Target enemy's defense is reduced 40%")) { creature.features.push(TargetEnemysDefenseIsReduced40Percent); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Drains life")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("-1(-2) enemy morale")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Acid attack")) {  }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Hates Angels")) { creature.features.push(Hates(vec!["Angel", "Archangel"])); }

    if creature.name == "Black Dragon" { creature.features.push(Hates(vec!["Titan"])); }

    if creature.name.contains("Elemental") { creature.features.push(Unliving); }
    if creature.name.contains("Gargoyle") { creature.features.push(Unliving); }
    if creature.name.contains("Golem") { creature.features.push(Unliving); }

    if creature.any_attr(&mut |it| it == "const_no_wall_penalty") {  }
    if creature.any_attr(&mut |it| it == "CATAPULT") {  }
    if creature.any_attr(&mut |it| it == "DOUBLE_WIDE") { creature.features.push(DoubleWide); }
    if creature.any_attr(&mut |it| it == "const_free_attack") {  }
    if creature.any_attr(&mut |it| it == "FLYING_ARMY") {  }
    if creature.any_attr(&mut |it| it == "KING_3") {  }
    if creature.any_attr(&mut |it| it == "IMMUNE_TO_MIND_SPELLS") {  }
    if creature.any_attr(&mut |it| it == "MULTI_HEADED") {  }
    if creature.any_attr(&mut |it| it == "const_raises_morale") {  }
    if creature.any_attr(&mut |it| it == "IS_UNDEAD") {  }
    if creature.any_attr(&mut |it| it == "SHOOTING_ARMY") { creature.features.push(Shoots); }
    if creature.any_attr(&mut |it| it == "SIEGE_WEAPON") {  }
    if creature.any_attr(&mut |it| it == "const_no_melee_penalty") {  }
    if creature.any_attr(&mut |it| it == "const_jousting") {  }
    if creature.any_attr(&mut |it| it == "IMMUNE_TO_FIRE_SPELLS") {  }
    if creature.any_attr(&mut |it| it == "const_lowers_morale") {  }
    if creature.any_attr(&mut |it| it == "IS_GHOST") {  }
    if creature.any_attr(&mut |it| it == "KING_1") {  }
    if creature.any_attr(&mut |it| it == "0") {  }
    if creature.any_attr(&mut |it| it == "KING_2") {  }
    if creature.any_attr(&mut |it| it == "const_two_attacks") {  }
    if creature.any_attr(&mut |it| it == "HAS_EXTENDED_ATTACK") {  }
}

pub fn code_gen(creatures: &[Creature]) {

    let mut flags: HashMap<String, String> = Default::default();
    for creature in creatures.iter() {
        // creature.any_ability(&mut |it| {
        //     let enum_name = ability_to_enum_name(it);
        //     let code = format!("if creature.any_ability(&mut |it| it == {:?}) {{ creature.ability_typed.push({}); }}", it, enum_name);
        //     flags.insert(enum_name, code);
        //     false
        // });
        creature.any_attr(&mut |it| {
            let enum_name = ability_to_enum_name(it);
            let code = format!("if creature.any_attr(&mut |it| it == {:?}) {{ creature.attrs_typed.push({}); }}", it, enum_name);
            flags.insert(enum_name, code);
            false
        });
    }
    for (enum_name, _) in flags.iter() {
        // println!("{},", enum_name);
    }
    for (_, code) in flags {
        println!("{}", code);
    }
}

fn ability_to_enum_name(s: &str) -> String {
    let mut n = String::new();
    let mut upper_next = false;
    if s.is_empty() || s.chars().next().unwrap().is_ascii_digit() {
        n.push('_');
    }
    for c in s.chars() {
        if c == ' ' {
            upper_next = true;
        } else if c == '\n' {
            upper_next = true;
        } else if c == '-' {
            n.push_str("Minus");
        } else if c == '%' {
            n.push_str("Percent");
        } else if c == '+' {
            n.push_str("Plus");
        } else if c == ':' {
            n.push_str("_");
        } else if c == '(' {
        } else if c == '/' {
        } else if c == '\'' {
        } else if c == ')' {
        } else if c == '&' {
            n.push_str("And");
        } else if upper_next {
            upper_next = false;
            for c in c.to_uppercase() {
                n.push(c);
            }
        } else {
            n.push(c);
        }
    }
    n
}