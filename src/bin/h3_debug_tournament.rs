use std::collections::HashMap;
use homm3_lab_rs::combat::{find_counter_count};
use homm3_lab_rs::creature::Creature;
use homm3_lab_rs::parse_creatures::parse_creatures;

fn main() {
    let creatures: HashMap<String, Creature> = parse_creatures(include_str!("../../data/h3/CRTRAIT0.txt"))
        .into_iter()
        .map(|it| (it.name.clone(), it))
        .collect();

    let a = creatures.get("Black Knight").unwrap();
    // let a = creatures.get("Bone Dragon").unwrap();
    let b = creatures.get("Archangel").unwrap();

    println!("a: {}", a.combat_info());
    println!("b: {}", b.combat_info());

    let clean = false;
    let result = find_counter_count(100, (1000, a), b, true, clean);

    let variants = result.iter().map(|it| format!("x{} with {:.01}%", it.closest_match_count, it.win_ratio * 100.0)).collect::<Vec<_>>();
    println!("{} x{} wins {}: {} (clean: {})", a.name, 1, b.name, variants.join(", "), clean);
}
