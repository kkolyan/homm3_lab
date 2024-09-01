use homm3_lab_rs::combat::{find_counter_count};
use homm3_lab_rs::parse_creatures::parse_creatures;

fn main() {
    let creatures = parse_creatures(include_str!("../../data/h3/CRTRAIT0.txt"));

    let inferno_melee = creatures.into_iter()
        .filter(|it| it.section == 3)
        // .filter(|it| it.name != "Magog" && it.name != "Gog")
        .collect::<Vec<_>>();

    for i in (1..inferno_melee.len()).rev() {
        for j in (0..i).rev() {
            let a = &inferno_melee[i];
            let b = &inferno_melee[j];

            let left_count = 10;

            let result = find_counter_count(100, (left_count, a), b, false);

            let variants = result.iter().map(|it| format!("x{} with {:.01}%", it.closest_match_count, it.win_ratio * 100.0)).collect::<Vec<_>>();
            println!("{} x{} wins {}: {}", a.name, left_count, b.name, variants.join(", "));
        }
    }
}
