use std::fs;
use homm3_lab_rs::parse_creatures::parse_creatures;

fn main() {
    let creatures = parse_creatures(include_str!("../../data/h3/CRTRAIT0.txt"));
    let mut s = String::new();
    for x in creatures.iter() {
        s.push_str(x.name.as_str());
        s.push_str("\n");
        // println!("{:?}", x);
    }
    fs::write("data/h3/structure1.txt", s).unwrap();
}