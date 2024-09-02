use std::collections::HashMap;
use crate::creature::{Creature, ResourceType};
use crate::parse_attrs_and_abilities::parse_attrs_and_abilities;

type Column<'a> = (&'a str, &'a dyn Fn(&mut Creature, &str));

#[allow(clippy::vec_init_then_push)]
pub fn parse_creatures(crtrait0_content: &str) -> Vec<Creature> {
    let mut columns: Vec<Column> = Default::default();
    columns.push(("Singular", &|it, s| it.name = s.into()));
    columns.push(("Plural", &|it, s| it.name_plural = s.into()));
    columns.push(("Wood", &|it, s| add_cost(&mut it.cost, ResourceType::Wood, s.parse().unwrap())));
    columns.push(("Mercury", &|it, s| add_cost(&mut it.cost, ResourceType::Mercury, s.parse().unwrap())));
    columns.push(("Ore", &|it, s| add_cost(&mut it.cost, ResourceType::Ore, s.parse().unwrap())));
    columns.push(("Sulfur", &|it, s| add_cost(&mut it.cost, ResourceType::Sulfur, s.parse().unwrap())));
    columns.push(("Crystal", &|it, s| add_cost(&mut it.cost, ResourceType::Crystal, s.parse().unwrap())));
    columns.push(("Gems", &|it, s| add_cost(&mut it.cost, ResourceType::Gems, s.parse().unwrap())));
    columns.push(("Gold", &|it, s| add_cost(&mut it.cost, ResourceType::Gold, s.parse().unwrap())));
    columns.push(("Fight Value", &|it, s| it.fight_value = s.parse().unwrap()));
    columns.push(("AI Value", &|it, s| it.ai_value = s.parse().unwrap()));
    columns.push(("Growth", &|it, s| it.growth = s.parse().unwrap()));
    columns.push(("Horde Growth", &|it, s| it.horde_growth = s.parse().unwrap()));
    columns.push(("Hit Points", &|it, s| it.health = s.parse().unwrap()));
    columns.push(("Speed", &|it, s| it.speed = s.parse().unwrap()));
    columns.push(("Attack", &|it, s| it.attack = s.parse().unwrap()));
    columns.push(("Defense", &|it, s| it.defence = s.parse().unwrap()));
    columns.push(("Low", &|it, s| it.damage_low = s.parse().unwrap()));
    columns.push(("High", &|it, s| it.damage_high = s.parse().unwrap()));
    columns.push(("Shots", &|it, s| it.shots = s.parse().unwrap()));
    columns.push(("Spells", &|it, s| it.spells = s.parse().unwrap()));
    columns.push(("Low", &|it, s| it.adv_map_low = s.parse().unwrap()));
    columns.push(("High", &|it, s| it.adv_map_high = s.parse().unwrap()));
    columns.push(("Ability Text", &|it, s| it.ability = s.into()));
    columns.push(("Attributes (Reference only, do not change these values)", &|it, s| it.attrs = s.into()));

    // let mut table: Vec<Vec<String>> = Default::default();

    let mut rem = crtrait0_content;

    // skip super-header
    {
        let (s, r) = rem.split_once('\n').unwrap();
        rem = r;
    }

    next_row(&mut rem, columns.len(), &mut |i, s| {
        assert_eq!(s, columns[i].0)
    });


    let mut creatures = Vec::new();

    let mut section = 0;
    let mut skips_remaining = 0;

    while !rem.is_empty() {
        let mut creature = Creature {
            section,
            ..Creature::default()
        };
        if skips_remaining > 0 {
            skips_remaining -= 1;
            next_row(&mut rem, columns.len(), &mut |column, s| {});
        } else {
            next_row(&mut rem, columns.len(), &mut |i, s| {
                if skips_remaining <= 0 {
                    // sections are separated with 3 empty rows (rows with all empty columns)
                    if i == 0 && s.trim().is_empty() {
                        section += 1;
                        skips_remaining = 2;
                    } else {
                        // println!("try to assign {} with {:?}", columns[i].0, s);
                        columns[i].1(&mut creature, s);
                    }
                }

            });
            if skips_remaining <= 0 {

                parse_attrs_and_abilities(&mut creature);
                creature.id = creatures.len() as u32 + 1;
                creatures.push(creature);
            }
        }


    }
    let mut id_by_name: HashMap<String, u32> = Default::default();
    for creature in creatures.iter_mut() {
        id_by_name.insert(creature.name.clone(), creature.id);
    }
    let hatemap = [
        (vec!["Angel", "Archangel"], vec!["Devil", "Arch Devil"]),
        (vec!["Black Dragon"], vec!["Titan"]),
        (vec!["Genie", "Master Genie"], vec!["Efreeti", "Efreet Sultan"]),
    ];
    for (a, b) in hatemap {
        for (a, b) in [(&a, &b), (&b, &a)] {
            for a in a {
                for b in b {
                    let a = creatures.get_mut(id_by_name[&a.to_string()] as usize - 1).unwrap();
                    a.hates.push(id_by_name[&b.to_string()]);
                }
            }
        }
    }
    creatures
}

fn next_row(data: &mut &str, columns: usize, handle_cell: &mut dyn FnMut(usize, &str)) {
    for i in 0..columns {
        let (s, rem_) = if i < columns - 1 {
            if data.starts_with('"') {
                data.strip_prefix('"').unwrap().split_once("\"\t").unwrap()
            } else {
                data.split_once('\t').unwrap()
            }
        } else {
            if data.starts_with('"') {
                data.strip_prefix('"').unwrap().split_once("\"\r\n").unwrap()
            } else {
                data.split_once("\r\n").unwrap()
            }
        };
        handle_cell(i, s);
        *data = rem_;
    }
}

fn add_cost(map: &mut HashMap<ResourceType, u32>, resource_type: ResourceType, value: u32) {
    let current = *map.get(&resource_type).unwrap_or(&0);
    let new = current + value;
    if new != 0 {
        map.insert(resource_type, new);
    }
}
