use std::collections::HashMap;
use std::{fs, thread};
use std::fmt::Display;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::atomic::Ordering::Relaxed;
use std::time::{SystemTime};
use crate::combat::{find_counter_count, UnboundU32};
use crate::creature::Creature;
use crate::parse_creatures::parse_creatures;
use crate::structure::parse_structure;


#[derive(Clone)]
struct CastleCreature {
    name: String,
    double_level: usize,
    castle: usize,
}

#[derive(Debug, Clone)]
pub struct FightResult {
    pub clean: bool,
    pub army_size: u32,
    pub counts: [UnboundU32; 2],
    pub win_rate: [f32; 2],
}

pub fn arrange_tournament(rounds: u32, crtrait0_txt: &str, structure_txt: &str) {
    calculate_tournament(rounds, crtrait0_txt);
    render(rounds, structure_txt);
}

#[derive(Copy, Clone)]
pub struct Task {
    pub i: usize,
    pub j: usize,
    pub army_size: u32,
    pub clean: bool,
}

pub fn prepare_tournament_table(crtrait0_txt: &str, army_sizes: &[u32], dry_varians: &[bool]) -> Vec<Task> {
    let creatures: Vec<Creature> = parse_creatures(crtrait0_txt)
        .into_iter()
        .filter(|it| !it.name.starts_with("NOT USED") && it.section < 10)
        .collect();

    let mut tasks = vec![];

    for i in (0..creatures.len()).rev() {
        for j in (0..creatures.len()).rev() {
            for clean in dry_varians {
                for army_size in army_sizes {
                    tasks.push(Task {i, j, army_size: *army_size, clean: *clean});
                }
            }
        }
    }
    tasks
}
pub fn calculate_tournament(rounds: u32, crtrait0_txt: &str) {

    let creatures: Vec<Creature> = parse_creatures(crtrait0_txt)
        .into_iter()
        .filter(|it| !it.name.starts_with("NOT USED") && it.section < 10)
        .collect();

    let army_sizes = [1, 10, 100];
    let dry_varians = [true, false];

    let tasks = prepare_tournament_table(crtrait0_txt, &army_sizes, &dry_varians);
    perform_tournament(rounds, rounds, &tasks, &creatures);
}

pub fn perform_tournament(rounds: u32, qualifier: impl Display, tasks: &[Task], creatures: &[Creature]) {

    let started_at = SystemTime::now();

    let mut results: HashMap<String, Vec<FightResult>> = Default::default();

    let mut last_printed_progress = String::new();


    let done = Arc::new(AtomicU32::new(0));

    let threads = (num_cpus::get() - 2).max(1);

    let mut task_partitions = vec![(); threads].into_iter().map(|_| Vec::new()).collect::<Vec<_>>();

    println!("tasks: {}, threads: {}", tasks.len(), threads);

    for (i, task) in tasks.iter().enumerate() {
        task_partitions[i % threads].push(*task);
    }

    let handles = task_partitions.into_iter()
        .map(|tasks| {
            let creatures = creatures.iter().cloned().collect::<Vec<_>>();
            let done = done.clone();
            thread::spawn(move || {
                let mut results: HashMap<String, Vec<FightResult>> = Default::default();

                for task in tasks {
                    let a = creatures.get(task.i).unwrap();
                    let b = creatures.get(task.j).unwrap();


                    // println!("{} vs {}", a.name, b.name);
                    let result = find_counter_count(rounds, (task.army_size, a), b, false, task.clean);


                    // let variants = result.iter().map(|it| format!("x{} with {:.01}%", it.closest_match_count, it.win_ratio * 100.0)).collect::<Vec<_>>();
                    // println!("{} x{} wins {}: {}", a.name, left_count, b.name, variants.join(", "));

                    let fight_result = FightResult {
                        clean: task.clean,
                        army_size: task.army_size,
                        counts: result.iter().map(|it| it.closest_match_count).collect::<Vec<_>>().try_into().unwrap(),
                        win_rate: result.iter().map(|it| it.win_ratio).collect::<Vec<_>>().try_into().unwrap(),
                    };
                    results.entry(format!("{} vs {}", a.name, b.name))
                        .or_default()
                        .push(fight_result);

                    loop {
                        let prev = done.load(Relaxed);
                        if done.compare_exchange(prev, prev + 1, Relaxed, Relaxed).is_ok() {
                            break;
                        }
                    }
                }
                results
            })
        })
        .collect::<Vec<_>>();

    let complete = Arc::new(AtomicBool::new(false));

    let total = tasks.len();

    let print_progress = {

        let qualifier = format!("{qualifier}");
        let complete = complete.clone();
        thread::spawn(move || {
            while !complete.load(Relaxed) {
                let progress = 1.0 * done.load(Relaxed) as f32 / total as f32;

                let progress_to_print = format!("{:.00}% (qualifier: {})", progress * 100.0, qualifier);

                if progress_to_print != last_printed_progress {
                    println!("{} (spent {:.03}s)", progress_to_print, SystemTime::now().duration_since(started_at).unwrap().as_secs_f32());
                    last_printed_progress = progress_to_print;
                }
            }
        })
    };

    handles.into_iter()
        .map(|it| it.join().unwrap())
        .for_each(|sub_results| {
            for (k, v) in sub_results {
                results.entry(k).or_default().extend(v);
            }
        });

    complete.store(true, Relaxed);
    print_progress.join().unwrap();

    let spent = SystemTime::now().duration_since(started_at).unwrap();

    println!("calculation complete in {:.03}s (rounds/pair: {}). rendering table...", spent.as_secs_f32(), rounds);

    let mut s = String::new();
    let columns: Vec<(&str, &dyn Fn(&str, &FightResult) -> String)> = vec![
        ("VS", &|vs, result| vs.to_string()),
        ("army_size", &|vs, result| result.army_size.to_string()),
        ("clean", &|vs, result| result.clean.to_string()),
        ("counts[0]", &|vs, result| result.counts[0].to_string()),
        ("counts[1]", &|vs, result| result.counts[1].to_string()),
        ("win_rate[0]", &|vs, result| result.win_rate[0].to_string()),
        ("win_rate[1]", &|vs, result| result.win_rate[1].to_string()),
    ];
    for (i, (column, _)) in columns.iter().enumerate() {
        s.push_str(column);
        if i < columns.len() - 1 {
            s.push_str("\t");
        } else {
            s.push_str("\n");
        }
    }
    for (vs, results) in results {
        for result in results {
            for (i, (_, func)) in columns.iter().enumerate() {
                s.push_str(func(&vs, &result).as_str());
                if i < columns.len() - 1 {
                    s.push_str("\t");
                } else {
                    s.push_str("\n");
                }
            }
        }
    }

    fs::write(format!("target/tournament.raw.{}.tsv", qualifier), s).unwrap();
}

pub fn load_fight_results(qualifier: impl Display) -> HashMap<String, Vec<FightResult>> {

    let mut results: HashMap<String, Vec<FightResult>> = Default::default();
    let results_str = fs::read_to_string(format!("target/tournament.raw.{}.tsv", qualifier)).unwrap();
    let mut results_str = results_str.lines();

    let header = results_str.next().unwrap().split("\t");
    let column_index_by_name: HashMap<&str, usize> = header.into_iter()
        .enumerate()
        .map(|(i, name)| (name, i))
        .collect();

    for line in results_str {
        let cells = line.split("\t").collect::<Vec<_>>();
        let vs = cells[column_index_by_name["VS"]];
        results.entry(vs.to_string())
            .or_default()
            .push(FightResult {
                clean: cells[column_index_by_name["clean"]] == "true",
                army_size: cells[column_index_by_name["army_size"]].parse().unwrap(),
                counts: [
                    cells[column_index_by_name["counts[0]"]].parse().unwrap(),
                    cells[column_index_by_name["counts[1]"]].parse().unwrap(),
                ],
                win_rate: [
                    cells[column_index_by_name["win_rate[0]"]].parse().unwrap(),
                    cells[column_index_by_name["win_rate[1]"]].parse().unwrap(),
                ],
            });
    }
    results
}

pub fn render(rounds: u32, structure_txt: &str) {
    let mut results: HashMap<String, Vec<FightResult>> = load_fight_results(rounds);

    for (_, results) in results.iter_mut() {
        results.sort_by_key(|it| if it.clean { 0 } else { 100000 } + it.army_size);
    }

    let castles = parse_structure(structure_txt);

    let mut castle_creatures = Vec::new();

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

    let sortings: Vec<(&str, &dyn Fn(&CastleCreature) -> i32)> = vec![
        ("-level,realm", &|it| -(it.double_level as i32) * 1000 + it.castle as i32),
        ("realm,level", &|it| it.castle as i32 * 1000 + it.double_level as i32),
    ];

    for (sorting_name, sorting_func) in sortings {
        castle_creatures.sort_by_key(|it| sorting_func(it));

        let mut s = String::new();
        s.push_str("\t");
        s.push_str("\t");
        s.push_str("\t");
        for (i, a) in castle_creatures.iter().enumerate() {
            s.push_str(format!("{}{}", 1 + a.double_level / 2, if a.double_level % 2 == 0 { "" } else { "+" }).as_str());
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
            s.push_str(format!("{}{}", 1 + a.double_level / 2, if a.double_level % 2 == 0 { "" } else { "+" }).as_str());
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

        fs::write(format!("target/tournament.{}.{}.tsv", sorting_name, rounds), s).unwrap();
    }
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
