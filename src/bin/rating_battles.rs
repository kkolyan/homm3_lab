use homm3_lab_rs::creature::Creature;
use homm3_lab_rs::generate_castle_chart::generate_castle_chart;
use homm3_lab_rs::parse_creatures::parse_creatures;
use homm3_lab_rs::tournament::{perform_tournament, Task};

fn main() {
    let crtrait0_txt = include_str!("../../data/h3/CRTRAIT0.txt");
    let creatures: Vec<Creature> = parse_creatures(crtrait0_txt)
        .into_iter()
        .filter(|it| !it.name.starts_with("NOT USED") && it.section < 10)
        .collect();
    let mut tasks = vec![];

    let bys = vec!["Imp", "Master Gremlin", "Archangel"];
    for by in bys {
        let j = creatures.iter().enumerate().find(|(i, it)| it.name == by).unwrap().0;
        for (i, _creature) in creatures.iter().enumerate() {
            if _creature.section >= 10 {
                continue;
            }
            tasks.push(Task {
                i,
                j,
                army_size: 10000,
                clean: false,
            });
        }
        perform_tournament(2, format!("vs {by}"), &tasks, &creatures);
        generate_castle_chart(by);
    }
}