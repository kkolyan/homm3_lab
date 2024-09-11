use homm3_lab_rs::combat::{play_match};
use homm3_lab_rs::creature::Creature;
use homm3_lab_rs::parse_creatures::parse_creatures;

fn main() {
    let creatures = parse_creatures(include_str!("../../data/h3/CRTRAIT0.txt"));
    for x in creatures.iter() {
        // println!("{:?}", x);
    }
    verbose_duel(&creatures, ("Dread Knight", 1000), ("Archangel", 301), false)
}
fn verbose_duel(creatures: &[Creature], a: (&str, u32), b: (&str, u32), clean: bool) {
    let a_c = creatures.iter().find(|it| it.name == a.0).unwrap();
    let b_c = creatures.iter().find(|it| it.name == b.0).unwrap();
    let result = play_match(1, (a.1, a_c), (b.1, b_c), true, clean);
    println!("{} x{} wins {} x{} in {:.01}%", a.0, a.1, b.0, b.1, result * 100.0)
}