use std::collections::HashMap;
use crate::creature::Creature;
use crate::creature::Ability::*;
use crate::creature::Attr::*;

pub fn parse_attrs_and_abilities(creature: &mut Creature) {
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Target enemy's defense is reduced 80%")) { creature.ability_typed.push(TargetEnemysDefenseIsReduced80Percent); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Vulnerable to ice")) { creature.ability_typed.push(VulnerableToIce); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Mind & fire immunity")) { creature.ability_typed.push(MindAndFireImmunity); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("cold vulnerability")) { creature.ability_typed.push(ColdVulnerability); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Shoot twice")) { creature.ability_typed.push(ShootTwice); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("-1 enemy luck")) { creature.ability_typed.push(Minus1EnemyLuck); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Merry")) { creature.ability_typed.push(Merry); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Shoots twice")) { creature.ability_typed.push(ShootsTwice); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Disease")) { creature.ability_typed.push(Disease); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Casts Protect from Fire")) { creature.ability_typed.push(CastsProtectFromFire); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Weakens enemies")) { creature.ability_typed.push(WeakensEnemies); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Strikes twice")) { creature.ability_typed.push(StrikesTwice); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("+1(+2) morale")) { creature.ability_typed.push(Plus1Plus2Morale); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("20% magic resistance")) { creature.ability_typed.push(_20PercentMagicResistance); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Immune to fire")) { creature.ability_typed.push(ImmuneToFire); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Mind spell immunity")) { creature.ability_typed.push(MindSpellImmunity); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Increase Hero's Knowledge by 1-3 per week")) { creature.ability_typed.push(IncreaseHerosKnowledgeBy1Minus3PerWeek); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Aura of magic resistance")) { creature.ability_typed.push(AuraOfMagicResistance); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("-1 to enemy morale")) { creature.ability_typed.push(Minus1ToEnemyMorale); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Death stare")) { creature.ability_typed.push(DeathStare); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Increase Hero's Defense Skill by 1-3 per week")) { creature.ability_typed.push(IncreaseHerosDefenseSkillBy1Minus3PerWeek); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Attacks all adjacent enemies")) { creature.ability_typed.push(AttacksAllAdjacentEnemies); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Protected against air spells")) { creature.ability_typed.push(ProtectedAgainstAirSpells); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("No walls or range penalty")) { creature.ability_typed.push(NoWallsOrRangePenalty); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Spits acid")) { creature.ability_typed.push(SpitsAcid); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Hero spells cost less")) { creature.ability_typed.push(HeroSpellsCostLess); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Drains enemy mana")) { creature.ability_typed.push(DrainsEnemyMana); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Dispels beneficial spells")) { creature.ability_typed.push(DispelsBeneficialSpells); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Retaliates twice")) { creature.ability_typed.push(RetaliatesTwice); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Damage from spells reduced 85%")) { creature.ability_typed.push(DamageFromSpellsReduced85Percent); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Fire spell Immunity")) { creature.ability_typed.push(FireSpellImmunity); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Lycanthropy (x2 if full moon)")) { creature.ability_typed.push(LycanthropyX2IfFullMoon); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Hates Black Dragons")) { creature.ability_typed.push(HatesBlackDragons); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Spellcaster: Bloodlust")) { creature.ability_typed.push(Spellcaster_Bloodlust); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Ignores defence")) { creature.ability_typed.push(IgnoresDefence); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Immune to ice")) { creature.ability_typed.push(ImmuneToIce); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Takes Soul")) { creature.ability_typed.push(TakesSoul); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("40% magic resistance")) { creature.ability_typed.push(_40PercentMagicResistance); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Lightning and firestorm vulnerability")) { creature.ability_typed.push(LightningAndFirestormVulnerability); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Ages enemy")) { creature.ability_typed.push(AgesEnemy); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Protected by Magic Mirror")) { creature.ability_typed.push(ProtectedByMagicMirror); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Can gift")) { creature.ability_typed.push(CanGift); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("3-headed attack")) { creature.ability_typed.push(_3MinusheadedAttack); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Ranged attacker")) { creature.ability_typed.push(RangedAttacker); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Good morale")) { creature.ability_typed.push(GoodMorale); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Extended Death stare")) { creature.ability_typed.push(ExtendedDeathStare); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("20% Block")) { creature.ability_typed.push(_20PercentBlock); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Fear")) { creature.ability_typed.push(Fear); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Petrifying attack")) { creature.ability_typed.push(PetrifyingAttack); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Attacks all adjacent enemies w/o retaliation")) { creature.ability_typed.push(AttacksAllAdjacentEnemiesWoRetaliation); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Fireball attack")) { creature.ability_typed.push(FireballAttack); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("No Range or Barrier penalties")) { creature.ability_typed.push(NoRangeOrBarrierPenalties); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("-1(-2) enemy luck")) { creature.ability_typed.push(Minus1Minus2EnemyLuck); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("No fear")) { creature.ability_typed.push(NoFear); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Spell immunity")) { creature.ability_typed.push(SpellImmunity); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Blinding attack")) { creature.ability_typed.push(BlindingAttack); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Paralyzing venom")) { creature.ability_typed.push(ParalyzingVenom); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Immune to spell levels 1-3")) { creature.ability_typed.push(ImmuneToSpellLevels1Minus3); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Immune to spell levels 1-4")) { creature.ability_typed.push(ImmuneToSpellLevels1Minus4); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Summon demons from")) { creature.ability_typed.push(SummonDemonsFrom); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Attacks siege walls")) { creature.ability_typed.push(AttacksSiegeWalls); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Group spell casters")) { creature.ability_typed.push(GroupSpellCasters); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Curse")) { creature.ability_typed.push(Curse); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("+1 Gem daily")) { creature.ability_typed.push(Plus1GemDaily); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Heals troops")) { creature.ability_typed.push(HealsTroops); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Berserk if full moon: days 14-16")) { creature.ability_typed.push(BerserkIfFullMoon_Days14Minus16); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Enemies cannot retaliate")) { creature.ability_typed.push(EnemiesCannotRetaliate); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Unlimited retaliations")) { creature.ability_typed.push(UnlimitedRetaliations); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Death cloud attack")) { creature.ability_typed.push(DeathCloudAttack); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("No melee penalty")) { creature.ability_typed.push(NoMeleePenalty); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Thunderclap")) { creature.ability_typed.push(Thunderclap); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Resurrects allies")) { creature.ability_typed.push(ResurrectsAllies); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Magic damper")) { creature.ability_typed.push(MagicDamper); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Fearsome")) { creature.ability_typed.push(Fearsome); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Spying")) { creature.ability_typed.push(Spying); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Damage from spells reduced 50%")) { creature.ability_typed.push(DamageFromSpellsReduced50Percent); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Offensive spell caster")) { creature.ability_typed.push(OffensiveSpellCaster); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Rebirth")) { creature.ability_typed.push(Rebirth); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Summon guards")) { creature.ability_typed.push(SummonGuards); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Hates Efreet")) { creature.ability_typed.push(HatesEfreet); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Strike and return")) { creature.ability_typed.push(StrikeAndReturn); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Ice Bolt attack")) { creature.ability_typed.push(IceBoltAttack); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Jousting bonus")) { creature.ability_typed.push(JoustingBonus); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Vulnerable to fire")) { creature.ability_typed.push(VulnerableToFire); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Protected against earth spells")) { creature.ability_typed.push(ProtectedAgainstEarthSpells); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Can blind")) { creature.ability_typed.push(CanBlind); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Meteor shower vulnerability")) { creature.ability_typed.push(MeteorShowerVulnerability); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Crystal generation")) { creature.ability_typed.push(CrystalGeneration); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("No Enemy retaliation")) { creature.ability_typed.push(NoEnemyRetaliation); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Protected against water spells")) { creature.ability_typed.push(ProtectedAgainstWaterSpells); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Damage from spells reduced 95%")) { creature.ability_typed.push(DamageFromSpellsReduced95Percent); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Sandwalkers")) { creature.ability_typed.push(Sandwalkers); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Resurrects twice")) { creature.ability_typed.push(ResurrectsTwice); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("+1(+2) luck")) { creature.ability_typed.push(Plus1Plus2Luck); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Increase Hero's Spell Power by 1-3 per week")) { creature.ability_typed.push(IncreaseHerosSpellPowerBy1Minus3PerWeek); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Positive luck")) { creature.ability_typed.push(PositiveLuck); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Immune to Fire Magic")) { creature.ability_typed.push(ImmuneToFireMagic); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Protected against fire spells")) { creature.ability_typed.push(ProtectedAgainstFireSpells); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Unlimited ammunition")) { creature.ability_typed.push(UnlimitedAmmunition); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Increases Hero's Attack Skill by 1-3 per week")) { creature.ability_typed.push(IncreasesHerosAttackSkillBy1Minus3PerWeek); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Damage from spells reduced 75%")) { creature.ability_typed.push(DamageFromSpellsReduced75Percent); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Regenerating")) { creature.ability_typed.push(Regenerating); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Sucks blood")) { creature.ability_typed.push(SucksBlood); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("+1 morale")) { creature.ability_typed.push(Plus1Morale); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Fire vulnerability")) { creature.ability_typed.push(FireVulnerability); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Immune to Blinding")) { creature.ability_typed.push(ImmuneToBlinding); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Poisonous")) { creature.ability_typed.push(Poisonous); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Immune to Champion charge bonus")) { creature.ability_typed.push(ImmuneToChampionChargeBonus); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Answers twice")) { creature.ability_typed.push(AnswersTwice); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Can regenerate")) { creature.ability_typed.push(CanRegenerate); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Ranged spell caster")) { creature.ability_typed.push(RangedSpellCaster); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Air shield")) { creature.ability_typed.push(AirShield); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("a dead ally")) { creature.ability_typed.push(aDeadAlly); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Immune to all spells")) { creature.ability_typed.push(ImmuneToAllSpells); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Stone gaze")) { creature.ability_typed.push(StoneGaze); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Ice immunity")) { creature.ability_typed.push(IceImmunity); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Attract dead souls")) { creature.ability_typed.push(AttractDeadSouls); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Spellcaster: Random benefit")) { creature.ability_typed.push(Spellcaster_RandomBenefit); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Death Blow attack")) { creature.ability_typed.push(DeathBlowAttack); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Fire Wall")) { creature.ability_typed.push(FireWall); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Magic channel")) { creature.ability_typed.push(MagicChannel); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Casts protection from Earth")) { creature.ability_typed.push(CastsProtectionFromEarth); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Breath attack")) { creature.ability_typed.push(BreathAttack); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Ignores obstacles")) { creature.ability_typed.push(IgnoresObstacles); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Fire Shield")) { creature.ability_typed.push(FireShield); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Binds enemies in place")) { creature.ability_typed.push(BindsEnemiesInPlace); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Curses enemies")) { creature.ability_typed.push(CursesEnemies); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Undead")) { creature.ability_typed.push(Undead); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Hates Devils")) { creature.ability_typed.push(HatesDevils); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Attack ages enemies")) { creature.ability_typed.push(AttackAgesEnemies); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Gets back")) { creature.ability_typed.push(GetsBack); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Slayer")) { creature.ability_typed.push(Slayer); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Rebirth always")) { creature.ability_typed.push(RebirthAlways); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Darkness")) { creature.ability_typed.push(Darkness); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("20% Magic Resist")) { creature.ability_typed.push(_20PercentMagicResist); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Hates Genies")) { creature.ability_typed.push(HatesGenies); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Lightning strike")) { creature.ability_typed.push(LightningStrike); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Target enemy's defense is reduced 40%")) { creature.ability_typed.push(TargetEnemysDefenseIsReduced40Percent); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Drains life")) { creature.ability_typed.push(DrainsLife); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("-1(-2) enemy morale")) { creature.ability_typed.push(Minus1Minus2EnemyMorale); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Acid attack")) { creature.ability_typed.push(AcidAttack); }
    if creature.any_ability(&mut |it| it.eq_ignore_ascii_case("Hates Angels")) { creature.ability_typed.push(HatesAngels); }

    if creature.any_attr(&mut |it| it == "const_no_wall_penalty") { creature.attrs_typed.push(const_no_wall_penalty); }
    if creature.any_attr(&mut |it| it == "CATAPULT") { creature.attrs_typed.push(CATAPULT); }
    if creature.any_attr(&mut |it| it == "DOUBLE_WIDE") { creature.attrs_typed.push(DOUBLE_WIDE); }
    if creature.any_attr(&mut |it| it == "const_free_attack") { creature.attrs_typed.push(const_free_attack); }
    if creature.any_attr(&mut |it| it == "FLYING_ARMY") { creature.attrs_typed.push(FLYING_ARMY); }
    if creature.any_attr(&mut |it| it == "KING_3") { creature.attrs_typed.push(KING_3); }
    if creature.any_attr(&mut |it| it == "IMMUNE_TO_MIND_SPELLS") { creature.attrs_typed.push(IMMUNE_TO_MIND_SPELLS); }
    if creature.any_attr(&mut |it| it == "MULTI_HEADED") { creature.attrs_typed.push(MULTI_HEADED); }
    if creature.any_attr(&mut |it| it == "const_raises_morale") { creature.attrs_typed.push(const_raises_morale); }
    if creature.any_attr(&mut |it| it == "IS_UNDEAD") { creature.attrs_typed.push(IS_UNDEAD); }
    if creature.any_attr(&mut |it| it == "SHOOTING_ARMY") { creature.attrs_typed.push(SHOOTING_ARMY); }
    if creature.any_attr(&mut |it| it == "SIEGE_WEAPON") { creature.attrs_typed.push(SIEGE_WEAPON); }
    if creature.any_attr(&mut |it| it == "const_no_melee_penalty") { creature.attrs_typed.push(const_no_melee_penalty); }
    if creature.any_attr(&mut |it| it == "const_jousting") { creature.attrs_typed.push(const_jousting); }
    if creature.any_attr(&mut |it| it == "IMMUNE_TO_FIRE_SPELLS") { creature.attrs_typed.push(IMMUNE_TO_FIRE_SPELLS); }
    if creature.any_attr(&mut |it| it == "const_lowers_morale") { creature.attrs_typed.push(const_lowers_morale); }
    if creature.any_attr(&mut |it| it == "IS_GHOST") { creature.attrs_typed.push(IS_GHOST); }
    if creature.any_attr(&mut |it| it == "KING_1") { creature.attrs_typed.push(KING_1); }
    if creature.any_attr(&mut |it| it == "0") { creature.attrs_typed.push(_0); }
    if creature.any_attr(&mut |it| it == "KING_2") { creature.attrs_typed.push(KING_2); }
    if creature.any_attr(&mut |it| it == "const_two_attacks") { creature.attrs_typed.push(const_two_attacks); }
    if creature.any_attr(&mut |it| it == "HAS_EXTENDED_ATTACK") { creature.attrs_typed.push(HAS_EXTENDED_ATTACK); }
}

pub fn code_gen(creatures: &[Creature]) {

    let mut flags: HashMap<String, String> = Default::default();
    for creature in creatures.iter() {
        // creature.any_ability(&mut |it| {
        //     let enum_name = ability_to_enum_name(it);
        //     let code = format!("if creature.any_ability(&mut |it| it == {:?}) {{ creature.ability_typed.push({}); }}", it, enum_name);
        //     flags.insert(enum_name, code);
        //     false
        // });
        creature.any_attr(&mut |it| {
            let enum_name = ability_to_enum_name(it);
            let code = format!("if creature.any_attr(&mut |it| it == {:?}) {{ creature.attrs_typed.push({}); }}", it, enum_name);
            flags.insert(enum_name, code);
            false
        });
    }
    for (enum_name, _) in flags.iter() {
        // println!("{},", enum_name);
    }
    for (_, code) in flags {
        println!("{}", code);
    }
}

fn ability_to_enum_name(s: &str) -> String {
    let mut n = String::new();
    let mut upper_next = false;
    if s.is_empty() || s.chars().next().unwrap().is_ascii_digit() {
        n.push('_');
    }
    for c in s.chars() {
        if c == ' ' {
            upper_next = true;
        } else if c == '\n' {
            upper_next = true;
        } else if c == '-' {
            n.push_str("Minus");
        } else if c == '%' {
            n.push_str("Percent");
        } else if c == '+' {
            n.push_str("Plus");
        } else if c == ':' {
            n.push_str("_");
        } else if c == '(' {
        } else if c == '/' {
        } else if c == '\'' {
        } else if c == ')' {
        } else if c == '&' {
            n.push_str("And");
        } else if upper_next {
            upper_next = false;
            for c in c.to_uppercase() {
                n.push(c);
            }
        } else {
            n.push(c);
        }
    }
    n
}