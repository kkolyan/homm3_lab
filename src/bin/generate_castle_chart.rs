use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use homm3_lab_rs::creature::Creature;
use homm3_lab_rs::ratings::rating_by;
use homm3_lab_rs::structure::{Castle, parse_structure};

#[derive(Clone)]
struct CastleCreature {
    name: String,
    double_level: usize,
    castle: usize,
    castle_name: String,
}
fn main() {
    generate_castle_chart("Imp");
    generate_castle_chart("Master Gremlin");
    generate_castle_chart("Archangel");
}
fn generate_castle_chart(by: &str) {

    let structure_txt = include_str!("../../data/h3/structure.txt");
    let by_imp = rating_by(by);

    let castles = parse_structure(structure_txt);

    let mut by_castle_and_double_level: HashMap<(String, usize), f32> = Default::default();
    for (i, castle) in castles.iter().enumerate() {
        if castle.name == "Neutral" {
            continue;
        }
        if castle.name != "Inferno" {
            // continue;
        }
        for (j, creature) in castle.creatures.iter().enumerate() {
            if let Some(rating) = by_imp.get(creature) {
                by_castle_and_double_level.insert((castle.name.clone(), j), *rating);
            }
        }
    }
    let mut double_levels = by_castle_and_double_level.keys()
        .map(|(_, double_level)| *double_level)
        .collect::<BTreeSet<_>>();

    let mut columns: Vec<(&str, Box<dyn Fn(usize) -> String>)> = vec![];
    columns.push(("Level", Box::new(|double_level| format!("{}{}", 1 + double_level / 2, if double_level % 2 == 0 {""} else {"+"}))));
    for castle in castles.iter() {
        if castle.name == "Neutral" {
            continue;
        }
        columns.push((castle.name.as_str(), Box::new(|double_level| {
            format!("{:03}", by_castle_and_double_level.get(&(castle.name.clone(), double_level)).unwrap_or(&0.0))
        })));
    }

    let mut s = String::new();
    for (i, (name, _)) in columns.iter().enumerate() {
        s.push_str(name);
        if i < columns.len() - 1 {
            s.push_str("\t");
        } else {
            s.push_str("\n");
        }
    }

    for double_level in double_levels {
        for (i, (_, getter)) in columns.iter().enumerate() {
            s.push_str(getter(double_level).as_str());
            if i < columns.len() - 1 {
                s.push_str("\t");
            } else {
                s.push_str("\n");
            }
        }
    }

    fs::write(format!("target/chart.{by}.tsv"), s).unwrap();
}