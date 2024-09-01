use homm3_lab_rs::parse_attrs_and_abilities::code_gen;
use homm3_lab_rs::parse_creatures::parse_creatures;

fn main() {
    let creatures = parse_creatures(include_str!("../../data/h3/CRTRAIT0.txt"));
    for x in creatures.iter() {
        // println!("{:?}", x);
    }
    code_gen(&creatures);
}
