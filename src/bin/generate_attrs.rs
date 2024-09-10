use std::collections::HashMap;
use std::fs;
use homm3_lab_rs::creature::{Creature, ResourceType};
use homm3_lab_rs::parse_creatures::parse_creatures;
use homm3_lab_rs::ratings::rating_by;
use homm3_lab_rs::structure::parse_structure;

#[derive(Clone)]
struct CastleCreature {
    name: String,
    double_level: usize,
    castle: usize,
    castle_name: String,
}

fn main() {
    let crtrait0_txt = include_str!("../../data/h3/CRTRAIT0.txt");
    let structure_txt = include_str!("../../data/h3/structure.txt");

    let creatures: HashMap<String, Creature> = parse_creatures(crtrait0_txt)
        .into_iter()
        .map(|it| (it.name.clone(), it))
        .collect();

    let mut castle_creatures = Vec::new();

    let castles = parse_structure(structure_txt);
    for (i, castle) in castles.iter().enumerate() {
        if castle.name == "Neutral" {
            continue;
        }
        if castle.name != "Inferno" {
            // continue;
        }
        for (j, creature) in castle.creatures.iter().enumerate() {
            castle_creatures.push(CastleCreature {
                name: creature.to_string(),
                double_level: j,
                castle: i,
                castle_name: castle.name.clone()
            });
        }
    }

    castle_creatures.sort_by_key(|it| it.castle as i32 * 1000 + it.double_level as i32);

    let by_imp = &rating_by("Imp");

    let rating_imp = |_cc: &CastleCreature, c: &Creature| {
        let name = c.name.to_string();
        format!("{:03}", by_imp[&name])
    };
    let mut columns: Vec<(&str, &dyn Fn(&CastleCreature, &Creature) -> String)> = vec![
        ("Realm", &|cc, c| cc.castle_name.to_string()),
        ("Level", &|cc, c| format!("{}{}", 1 + cc.double_level / 2, if cc.double_level % 2 == 0 {""} else {"+"})),
        ("Name", &|cc, c| c.name.to_string()),
        ("Gold", &|cc, c| c.cost[&ResourceType::Gold].to_string()),
        ("Attack", &|cc, c| c.attack.to_string()),
        ("Defence", &|cc, c| c.defence.to_string()),
        ("Damage Low", &|cc, c| c.damage_low.to_string()),
        ("Damage High", &|cc, c| c.damage_high.to_string()),
        ("Speed", &|cc, c| c.speed.to_string()),
        ("Health", &|cc, c| c.health.to_string()),
        ("R/Imp", &rating_imp),
        ("Features", &|cc, c| {
            let mut s = String::new();
            s.push_str("\"");
            for (i, feature) in c.features.iter().enumerate() {
                s.push_str(format!("{}", feature).as_str());
                if i < c.features.len() - 1 {
                    s.push_str("\n")
                }
            }
            s.push_str("\"");
            s
        }),
    ];

    let mut s = String::new();
    for (i, (name, _)) in columns.iter().enumerate() {
        s.push_str(name);
        if i < columns.len() - 1 {
            s.push_str("\t");
        } else {
            s.push_str("\n");
        }
    }

    for castle_creature in castle_creatures.iter() {
        let creature = creatures.get(&castle_creature.name).unwrap();
        for (i, (_, getter)) in columns.iter().enumerate() {
            s.push_str(getter(castle_creature, creature).as_str());
            if i < columns.len() - 1 {
                s.push_str("\t");
            } else {
                s.push_str("\n");
            }
        }
    }

    fs::write("target/attrs.tsv", s).unwrap();

}