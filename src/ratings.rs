use std::collections::HashMap;
use crate::combat::UnboundU32;
use crate::tournament::load_fight_results;

pub fn rating_by(by: &str) -> HashMap<String, f32> {
    let fight_results = load_fight_results(format!("vs {by}"));

    let mut ratings: HashMap<String, f32> = Default::default();
    for (vs, results) in fight_results {
        let (attacker, defender) = vs.split_once(" vs ").unwrap();
        if defender != by {
            continue
        }
        for result in results {
            let defender_count = match result.counts[0] {
                UnboundU32::Value(it) => it,
                UnboundU32::Inf => panic!()
            };
            ratings.insert(attacker.to_string(), defender_count as f32 / result.army_size as f32);
        }
    }

    ratings
}