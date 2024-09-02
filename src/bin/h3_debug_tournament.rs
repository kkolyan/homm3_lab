use std::collections::HashMap;
use homm3_lab_rs::combat::{find_counter_count};
use homm3_lab_rs::creature::Creature;
use homm3_lab_rs::parse_creatures::parse_creatures;

fn main() {
    let creatures: HashMap<String, Creature> = parse_creatures(include_str!("../../data/h3/CRTRAIT0.txt"))
        .into_iter()
        .map(|it| (it.name.clone(), it))
        .collect();

    let a = creatures.get("Gnoll").unwrap();
    let b = creatures.get("Pikeman").unwrap();

    let clean = true;
    let result = find_counter_count(1000, (1, a), b, true, clean);

    let variants = result.iter().map(|it| format!("x{} with {:.01}%", it.closest_match_count, it.win_ratio * 100.0)).collect::<Vec<_>>();
    println!("{} x{} wins {}: {} (clean: {})", a.name, 1, b.name, variants.join(", "), clean);
}
