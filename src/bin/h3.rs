use homm3_lab_rs::combat::{find_counter_count, play_match};
use homm3_lab_rs::creature::Creature;
use homm3_lab_rs::parse_creatures::parse_creatures;

fn main() {
    let creatures = parse_creatures(include_str!("../../data/h3/CRTRAIT0.txt"));
    for x in creatures.iter() {
        // println!("{:?}", x);
    }
    // let arch_devil = creatures.iter().find(|it| it.name == "Arch Devil").unwrap();
    // let efreet_sultan = creatures.iter().find(|it| it.name == "Efreet Sultan").unwrap();
    // play_match(10000, (100, arch_devil), (358, efreet_sultan));
    // let result = find_counter_count(100, (100, arch_devil), efreet_sultan);
    // println!("{:?}", result)
    //code_gen(&creatures);

    let mut inferno_melee = creatures.into_iter()
        .filter(|it| it.section == 3)
        // .filter(|it| it.name != "Magog" && it.name != "Gog")
        .collect::<Vec<_>>();

    for i in (1..inferno_melee.len()).rev() {
        for j in (0..i).rev() {
            let a = &inferno_melee[i];
            let b = &inferno_melee[j];

            let left_count = 10;

            let result = find_counter_count(100, (left_count, a), b);

            let variants = result.iter().map(|it| format!("x{} with {:.01}%", it.closest_match_count, it.win_ratio * 100.0)).collect::<Vec<_>>();
            println!("{} x{} wins {}: {}", a.name, left_count, b.name, variants.join(", "));
        }
    }

    // verbose_duel(&creatures, ("Gog", 1), ("Imp", 3))
}
fn verbose_duel(creatures: &[Creature], a: (&str, u32), b: (&str, u32)) {
    let a_c = creatures.iter().find(|it| it.name == a.0).unwrap();
    let b_c = creatures.iter().find(|it| it.name == b.0).unwrap();
    let result = play_match(1, (a.1, a_c), (b.1, b_c), true);
    println!("{} x{} wins {} x{} in {:.01}%", a.0, a.1, b.0, b.1, result * 100.0)
}