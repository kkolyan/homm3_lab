use std::collections::HashMap;
use std::io;
use io::Write;
use std::fmt::{Display, Formatter};

#[derive(Default, Clone, Debug)]
pub struct Creature {
    pub section: u32,
    pub id: u32,
    pub name: String,
    pub name_plural: String,
    pub cost: HashMap<ResourceType, u32>,
    pub fight_value: u32,
    pub ai_value: u32,
    pub growth: u32,
    pub horde_growth: u32,
    pub health: u32,
    pub speed: u32,
    pub attack: u32,
    pub defence: u32,
    pub damage_low: u32,
    pub damage_high: u32,
    pub shots: u32,
    pub spells: u32,
    pub adv_map_low: u32,
    pub adv_map_high: u32,
    pub ability: String,
    pub ability_typed: Vec<Ability>,
    pub attrs: String,
    pub attrs_typed: Vec<Attr>,
    pub hates: Vec<u32>,
    pub features: Vec<Feature>,
}

impl Creature {
    pub fn has_feature(&self, feature: Feature) -> bool {
        self.features.contains(&feature)
    }

    pub fn has_any_feature<const T: usize>(&self, feature: [Feature; T]) -> bool {
        feature.into_iter().any(|feature| self.has_feature(feature))
    }

    pub fn combat_info(&self) -> String {
        let mut s: Vec<u8> = Default::default();
        write!( s, "Name: {}, Attack: {}, Defence: {}, Dam: {}-{}, Speed: {}, abils: {:?}, attrs: {:?}", self.name, self.attack, self.defence, self.damage_low, self.damage_high, self.speed, self.ability_typed, self.attrs_typed).unwrap();
        String::from_utf8(s).unwrap()
    }
    pub fn any_ability(&self, callback: &mut dyn FnMut(&str) -> bool) -> bool {
        for x in self.ability.split('.') {
            let x = x.trim();
            for x in x.split(',') {
                let x = x.trim();
                for x in x.split("\n") {
                    let x = x.trim();
                    if !x.is_empty() && callback(x) {
                        return true;
                    }
                }
            }
        }
        false
    }
    pub fn any_attr(&self, callback: &mut dyn FnMut(&str) -> bool) -> bool {
        for x in self.attrs.split('|') {
            let x = x.trim();
            let x = x.trim();
            if !x.is_empty() && callback(x) {
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Debug, Copy, Hash, Eq, PartialEq)]
pub enum Attr {
}

#[derive(Clone, Debug, Copy, Hash, Eq, PartialEq)]
pub enum Ability {

}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum ResourceType {
    Wood,
    Mercury,
    Ore,
    Sulfur,
    Crystal,
    Gems,
    Gold,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Feature {
    Ages,

    Shoots,
    ShootsTwice,
    StrikesTwice,

    DoubleWide,
    DeathStare,
    RetaliatesTwice,
    Poisonous,
    UnlimitedRetaliations,
    EnemiesCannotRetaliate,
    TargetEnemysDefenseIsReduced80Percent,
    TargetEnemysDefenseIsReduced40Percent,
    NoMeleePenalty,
    FireShield,
    Undead,
    Unliving,
    ImmuneToAging,
    Hates(Vec<&'static str>),
}

impl Display for Feature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Feature::Hates(hates) = self {
            return write!(f, "Hates( {} )", hates.join(", "));
        }
        write!(f, "{:?}", self)
    }
}

