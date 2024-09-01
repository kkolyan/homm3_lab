use homm3_lab_rs::combat::play_match;
use homm3_lab_rs::parse_creatures::parse_creatures;

fn main() {
    let creatures = parse_creatures(include_str!("../../data/h3/CRTRAIT0.txt"));
    for x in creatures.iter() {
        // println!("{:?}", x);
    }
    let arch_devil = creatures.iter().find(|it| it.name == "Arch Devil").unwrap();
    let efreet_sultan = creatures.iter().find(|it| it.name == "Efreet Sultan").unwrap();
    play_match(10000, (100, arch_devil), (358, efreet_sultan));
    //code_gen(&creatures);
}