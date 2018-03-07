
use std::collections::HashSet;
#[derive(Debug)]
pub struct Hero {
    pub level: u32,
    pub skills: HashSet<Skill>,
    pub movement_points: u8,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Skill {
    pub kill_type: SkillType,
    pub evel: u8
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum SkillType {
    Movement,
    Archery,
    Diplomacy,
    Estates,
    Leadership,
    Luck,
    Mana,
    Navigation,
    Necromancy,
    Pathfinding,
    Scouting,
    Wisdom,
}
