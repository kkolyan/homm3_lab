use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct Creature {
    pub section: u32,
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
}

impl Creature {
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
    DOUBLE_WIDE,
    const_lowers_morale,
    IMMUNE_TO_MIND_SPELLS,
    MULTI_HEADED,
    const_no_wall_penalty,
    IS_UNDEAD,
    KING_3,
    FLYING_ARMY,
    const_two_attacks,
    _0,
    const_free_attack,
    CATAPULT,
    KING_2,
    const_jousting,
    IMMUNE_TO_FIRE_SPELLS,
    IS_GHOST,
    const_raises_morale,
    SIEGE_WEAPON,
    KING_1,
    const_no_melee_penalty,
    SHOOTING_ARMY,
    HAS_EXTENDED_ATTACK,

}

#[derive(Clone, Debug, Copy, Hash, Eq, PartialEq)]
pub enum Ability {
    AttacksAllAdjacentEnemiesWoRetaliation,
    ShootTwice,
    BerserkIfFullMoon_Days14Minus16,
    NoMeleePenalty,
    GoodMorale,
    NoRangeOrBarrierPenalties,
    BindsEnemiesInPlace,
    Plus1Plus2Morale,
    Merry,
    AttacksSiegeWalls,
    MindSpellImmunity,
    SpellImmunity,
    RetaliatesTwice,
    AirShield,
    GroupSpellCasters,
    Undead,
    RangedAttacker,
    ImmuneToAllSpells,
    ParalyzingVenom,
    JoustingBonus,
    DeathStare,
    FireSpellImmunity,
    NoFear,
    IncreaseHerosKnowledgeBy1Minus3PerWeek,
    Thunderclap,
    _20PercentMagicResist,
    DamageFromSpellsReduced95Percent,
    Sandwalkers,
    DrainsLife,
    ImmuneToBlinding,
    WeakensEnemies,
    StrikeAndReturn,
    VulnerableToIce,
    DrainsEnemyMana,
    LightningStrike,
    Spellcaster_RandomBenefit,
    Rebirth,
    Plus1Morale,
    CanBlind,
    EnemiesCannotRetaliate,
    AttacksAllAdjacentEnemies,
    TargetEnemysDefenseIsReduced80Percent,
    SpitsAcid,
    Spying,
    Minus1Minus2EnemyMorale,
    Minus1ToEnemyMorale,
    AttackAgesEnemies,
    Fearsome,
    FireWall,
    DamageFromSpellsReduced50Percent,
    ProtectedAgainstEarthSpells,
    FireballAttack,
    BlindingAttack,
    IceImmunity,
    PetrifyingAttack,
    _20PercentMagicResistance,
    MagicDamper,
    aDeadAlly,
    UnlimitedAmmunition,
    Spellcaster_Bloodlust,
    ProtectedAgainstFireSpells,
    ImmuneToSpellLevels1Minus4,
    ExtendedDeathStare,
    GetsBack,
    Minus1EnemyLuck,
    NoEnemyRetaliation,
    TargetEnemysDefenseIsReduced40Percent,
    HatesBlackDragons,
    ProtectedAgainstAirSpells,
    MagicChannel,
    OffensiveSpellCaster,
    ImmuneToFireMagic,
    RangedSpellCaster,
    HatesGenies,
    CrystalGeneration,
    IncreasesHerosAttackSkillBy1Minus3PerWeek,
    CastsProtectionFromEarth,
    StrikesTwice,
    ImmuneToChampionChargeBonus,
    BreathAttack,
    SucksBlood,
    Slayer,
    StoneGaze,
    CanGift,
    HatesEfreet,
    CastsProtectFromFire,
    ImmuneToSpellLevels1Minus3,
    ImmuneToFire,
    SummonDemonsFrom,
    Darkness,
    DeathCloudAttack,
    DamageFromSpellsReduced75Percent,
    LightningAndFirestormVulnerability,
    Fear,
    IgnoresDefence,
    HeroSpellsCostLess,
    AnswersTwice,
    AgesEnemy,
    NoWallsOrRangePenalty,
    _20PercentBlock,
    _40PercentMagicResistance,
    TakesSoul,
    AcidAttack,
    CursesEnemies,
    FireShield,
    Minus1Minus2EnemyLuck,
    AttractDeadSouls,
    IceBoltAttack,
    ResurrectsAllies,
    DispelsBeneficialSpells,
    Poisonous,
    PositiveLuck,
    ProtectedByMagicMirror,
    Plus1Plus2Luck,
    UnlimitedRetaliations,
    ProtectedAgainstWaterSpells,
    HealsTroops,
    LycanthropyX2IfFullMoon,
    Curse,
    DamageFromSpellsReduced85Percent,
    Regenerating,
    Plus1GemDaily,
    SummonGuards,
    FireVulnerability,
    MeteorShowerVulnerability,
    CanRegenerate,
    IncreaseHerosDefenseSkillBy1Minus3PerWeek,
    IgnoresObstacles,
    VulnerableToFire,
    ImmuneToIce,
    RebirthAlways,
    _3MinusheadedAttack,
    HatesDevils,
    MindAndFireImmunity,
    ColdVulnerability,
    ResurrectsTwice,
    HatesAngels,
    DeathBlowAttack,
    ShootsTwice,
    AuraOfMagicResistance,
    Disease,
    IncreaseHerosSpellPowerBy1Minus3PerWeek,

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

