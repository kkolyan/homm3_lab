use std::collections::HashMap;
use homm3_lab_rs::combat::{find_counter_count, play_match};
use homm3_lab_rs::creature::Creature;
use homm3_lab_rs::parse_creatures::parse_creatures;
use homm3_lab_rs::structure::parse_structure;

fn main() {
    let creatures: HashMap<String, Creature> = parse_creatures(include_str!("../../data/h3/CRTRAIT0.txt"))
        .into_iter()
        .map(|it| (it.name.clone(), it))
        .collect();

    let a = creatures.get("Magma Elemental").unwrap();
    let b = creatures.get("Chaos Hydra").unwrap();

    let left_count = 10;

    println!("{} vs {}", a.name, b.name);
    let result = find_counter_count(10, (left_count, a), b, true);

    let variants = result.iter().map(|it| format!("x{} with {:.01}%", it.closest_match_count, it.win_ratio * 100.0)).collect::<Vec<_>>();
    println!("{} x{} wins {}: {}", a.name, left_count, b.name, variants.join(", "));
}
