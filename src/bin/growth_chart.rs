use std::collections::HashMap;
use std::{fmt, fs};
use std::io::Write;
use std::iter::Map;
use std::ops::Div;
use homm3_lab_rs::creature::Creature;
use homm3_lab_rs::generate_castle_chart::generate_castle_chart;
use homm3_lab_rs::parse_creatures::parse_creatures;
use homm3_lab_rs::structure::parse_structure;
use homm3_lab_rs::tournament::{perform_tournament, Task};
use homm3_lab_rs::tsv;
use homm3_lab_rs::tsv::ListTable;

fn main() {
    let crtrait0_txt = include_str!("../../data/h3/CRTRAIT0.txt");
    let structure = include_str!("../../data/h3/structure.txt");


    let creatures: HashMap<String, Creature> = parse_creatures(crtrait0_txt)
        .into_iter()
        .filter(|it| !it.name.starts_with("NOT USED") && it.section < 10)
        .map(|it| (it.name.to_string(), it))
        .collect();

    let structure = parse_structure(structure)
        .into_iter()
        .filter(|it| it.name != "Neutral")
        .collect::<Vec<_>>();

    let mut table: ListTable<usize> = Default::default();
    table.add_column("Level", |double_level, w| if double_level % 2 == 0 { write!(w, "{}", double_level / 2) } else { write!(w, "{}+", double_level / 2) });

    for (i, castle) in structure.iter().enumerate() {
        let structure = &structure;
        let creatures = &creatures;
        table.add_column(castle.name.to_string(), move |double_level: &usize, w: &mut dyn fmt::Write| {
            let i = i;
            let castle = &structure[i];
            let name = &castle.creatures[*double_level];
            let creature = &creatures[name];

            write!(w, "{}", creature.growth)
        })
    }

    for i in 0..(7 * 2) {
        table.add_row(i);
    }

    let s = tsv::format_tsv(&table).unwrap();
    fs::write("target/growth_chart.tsv", s).unwrap();
}