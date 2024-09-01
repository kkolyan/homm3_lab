use homm3_lab_rs::structure::parse_structure;

fn main() {
    let structure = parse_structure(include_str!("../../data/h3/structure.txt"));
    for castle in structure {
        println!("{}", castle.name);
        for creature in castle.creatures {
            println!("  {}", creature.name);
        }
    }
}