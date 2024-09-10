use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use homm3_lab_rs::generate_castle_chart::generate_castle_chart;
use homm3_lab_rs::ratings::rating_by;
use homm3_lab_rs::structure::{Castle, parse_structure};

#[derive(Clone)]
struct CastleCreature {
    name: String,
    double_level: usize,
    castle: usize,
    castle_name: String,
}
fn main() {
    generate_castle_chart("Imp");
    generate_castle_chart("Master Gremlin");
    generate_castle_chart("Archangel");
}
