use std::collections::HashMap;
use std::fs;
use std::time::{SystemTime};
use crate::combat::{find_counter_count, UnboundU32};
use crate::creature::Creature;
use crate::parse_creatures::parse_creatures;
use crate::structure::parse_structure;


struct CastleCreature {
    name: String,
    double_level: usize,
    castle: usize,
}

struct FightResult {
    clean: bool,
    army_size: u32,
    counts: [UnboundU32; 2],
    win_rate: [f32; 2],
}

pub fn arrange_tournament(rounds: u32, crtrait0_txt: &str, structure_txt: &str) {
    let creatures: HashMap<String, Creature> = parse_creatures(crtrait0_txt)
        .into_iter()
        .map(|it| (it.name.clone(), it))
        .collect();

    let mut castle_creatures = Vec::new();

    let started_at = SystemTime::now();

    let castles = parse_structure(structure_txt);
    for (i, castle) in castles.iter().enumerate() {
        if castle.name == "Neutral" {
            continue;
        }
        if castle.name != "Inferno" {
            // continue;
        }
        for (j, creature) in castle.creatures.iter().enumerate() {
            castle_creatures.push(CastleCreature {
                name: creature.to_string(),
                double_level: j,
                castle: i,
            });
        }
    }

    castle_creatures.sort_by_key(|it| it.castle as i32 - it.double_level as i32 * 100);

    let mut results: HashMap<String, Vec<FightResult>> = Default::default();

    let mut last_printed_progress = String::new();

    let total = castle_creatures.len() * (castle_creatures.len() - 1);

    let mut done = 0;

    let army_sizes = [1, 10, 100];
    let dry_varians = [true, false];

    for i in (0..castle_creatures.len()).rev() {
        for j in (0..castle_creatures.len()).rev() {
            let a = creatures.get(&castle_creatures[i].name).unwrap();
            let b = creatures.get(&castle_creatures[j].name).unwrap();

            for clean in dry_varians {
                if i == j && !clean {
                    // clean win rates makes sense even for the same type of army.
                    continue;
                }
                for army_size in army_sizes {


                    // println!("{} vs {}", a.name, b.name);
                    let result = find_counter_count(rounds, (army_size, a), b, false, clean);


                    // let variants = result.iter().map(|it| format!("x{} with {:.01}%", it.closest_match_count, it.win_ratio * 100.0)).collect::<Vec<_>>();
                    // println!("{} x{} wins {}: {}", a.name, left_count, b.name, variants.join(", "));

                    let fight_result = FightResult {
                        clean,
                        army_size,
                        counts: result.iter().map(|it| it.closest_match_count).collect::<Vec<_>>().try_into().unwrap(),
                        win_rate: result.iter().map(|it| it.win_ratio).collect::<Vec<_>>().try_into().unwrap(),
                    };
                    results.entry(format!("{} vs {}", a.name, b.name))
                        .or_default()
                        .push(fight_result);

                    done += 1;

                    let progress = 1.0 * done as f32 / total as f32 / army_sizes.len() as f32 / dry_varians.len() as f32;

                    let progress_to_print = format!("{:.00}%", progress * 100.0);

                    if progress_to_print != last_printed_progress {
                        println!("{} (spent {:.03}s)", progress_to_print, SystemTime::now().duration_since(started_at).unwrap().as_secs_f32());
                        last_printed_progress = progress_to_print;
                    }
                }
            }
        }
    }

    let spent = SystemTime::now().duration_since(started_at).unwrap();

    println!("calculation complete in {:.03}s (rounds/pair: {}). rendering table...", spent.as_secs_f32(), rounds);

    let mut s = String::new();
    s.push_str("\t");
    s.push_str("\t");
    s.push_str("\t");
    for (i, a) in castle_creatures.iter().enumerate() {
        s.push_str(format!("{}{}", 1 + a.double_level / 2, if a.double_level % 2 == 0 {""} else {"+"}).as_str());
        if i < castle_creatures.len() - 1 {
            s.push_str("\t");
        }
    }
    s.push_str("\n");
    s.push_str("\t");
    s.push_str("\t");
    s.push_str("\t");
    for (i, a) in castle_creatures.iter().enumerate() {
        s.push_str(castles.get(a.castle).unwrap().name.as_str());
        if i < castle_creatures.len() - 1 {
            s.push_str("\t");
        }
    }
    s.push_str("\n");
    s.push_str("\t");
    s.push_str("\t");
    s.push_str("\t");
    for (i, a) in castle_creatures.iter().enumerate() {
        s.push_str(a.name.as_str());
        if i < castle_creatures.len() - 1 {
            s.push_str("\t");
        }
    }
    s.push_str("\n");

    for a in castle_creatures.iter() {
        s.push_str(format!("{}{}", 1 + a.double_level / 2, if a.double_level % 2 == 0 {""} else {"+"}).as_str());
        s.push_str("\t");
        s.push_str(castles.get(a.castle).unwrap().name.as_str());
        s.push_str("\t");
        s.push_str(a.name.as_str());
        s.push_str("\t");

        for (i, b) in castle_creatures.iter().enumerate() {
            if let Some(result) = results.get(&format!("{} vs {}", a.name, b.name)) {
                render_cell(&mut s, result);
            }
            if i < castle_creatures.len() - 1 {
                s.push_str("\t");
            }
        }
        s.push_str("\n");
    }

    fs::write(format!("target/tournament.{}.tsv", rounds), s).unwrap();
}

fn render_cell(s: &mut String, result: &[FightResult]) {
    s.push_str("\"");
    let lines = result.len();
    for (i, result) in result.iter().enumerate() {
        let mut msg = if result.counts[0] == result.counts[1] && result.win_rate[0] == result.win_rate[1] {
            format!("{} vs {}: {:.00}%", result.army_size, result.counts[0], result.win_rate[0] * 100.0)
        } else {
            format!("{} vs {}-{}: {:.00}%-{:.00}%", result.army_size, result.counts[0], result.counts[1], result.win_rate[0] * 100.0, result.win_rate[1] * 100.0)
        };

        s.push_str(msg.as_str());
        if result.clean {
            s.push_str(" (Clean)");
        }

        if i != lines - 1 {
            s.push_str("\n");
        }
    }
    s.push_str("\"");
}
