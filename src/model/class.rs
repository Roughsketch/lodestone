use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug, Fail)]
#[fail(display = "Invalid class type '{}'", _0)]
pub struct ClassTypeParseError(String);

/// An enum over the types of classes or jobs that are available.
/// 
/// In the case of unlocking a job, the higher level one is preferred.
/// For example, after unlocking Paladin, the class type will return
/// Paladin instead of Gladiator.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ClassType {
    //  Tank
    Paladin,
    Gladiator,
    Warrior,
    Marauder,
    DarkKnight,
    //  Healer
    WhiteMage,
    Conjurer,
    Scholar,
    Astrologian,
    //  Damage
    Monk,
    Pugilist,
    Dragoon,
    Lancer,
    Ninja,
    Rogue,
    Samurai,
    Bard,
    Archer,
    Machinist,
    BlackMage,
    Thaumaturge,
    Summoner,
    Arcanist,
    RedMage,
    BlueMage,
    //  Crafting
    Carpenter,
    Blacksmith,
    Armorer,
    Goldsmith,
    Leatherworker,
    Weaver,
    Alchemist,
    Culinarian,
    //  Gathering
    Miner,
    Botanist,
    Fisher,
}

/// Takes a string from lodestone and converts it to a ClassType.
/// Can take either the full name, or its common abbreviation as
/// shown on gear and the conversion is case insensitive.
/// 
/// For example, `paladin` and `PLD` will both convert to 
/// `ClassType::Paladin`
impl FromStr for ClassType {
    type Err = ClassTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_uppercase() {
            "PALADIN"       | "PLD" => Ok(ClassType::Paladin),
            "GLADIATOR"     | "GLD" => Ok(ClassType::Gladiator),
            "WARRIOR"       | "WAR" => Ok(ClassType::Warrior),
            "MARAUDER"      | "MRD" => Ok(ClassType::Marauder),
            "DARK KNIGHT"   | "DRK" => Ok(ClassType::DarkKnight),
            "WHITE MAGE"    | "WHM" => Ok(ClassType::WhiteMage),
            "CONJURER"      | "CNJ" => Ok(ClassType::Conjurer),
            "SCHOLAR"       | "SCH" => Ok(ClassType::Scholar),
            "ASTROLOGIAN"   | "AST" => Ok(ClassType::Astrologian),
            "MONK"          | "MNK" => Ok(ClassType::Monk),
            "PUGILIST"      | "PUG" => Ok(ClassType::Pugilist),
            "DRAGOON"       | "DRG" => Ok(ClassType::Dragoon),
            "LANCER"        | "LNC" => Ok(ClassType::Lancer),
            "NINJA"         | "NIN" => Ok(ClassType::Ninja),
            "ROGUE"         | "ROG" => Ok(ClassType::Rogue),
            "SAMURAI"       | "SAM" => Ok(ClassType::Samurai),
            "BARD"          | "BRD" => Ok(ClassType::Bard),
            "ARCHER"        | "ARC" => Ok(ClassType::Archer),
            "MACHINIST"     | "MCH" => Ok(ClassType::Machinist),
            "BLACK MAGE"    | "BLM" => Ok(ClassType::BlackMage),
            "THAUMATURGE"   | "THM" => Ok(ClassType::Thaumaturge),
            "SUMMONER"      | "SMN" => Ok(ClassType::Summoner),
            "ARCANIST"      | "ACN" => Ok(ClassType::Arcanist),
            "RED MAGE"      | "RDM" => Ok(ClassType::RedMage),
            "BLUE MAGE (LIMITED JOB)" | "BLU" => Ok(ClassType::BlueMage),
            "CARPENTER"     | "CRP" => Ok(ClassType::Carpenter),
            "BLACKSMITH"    | "BSM" => Ok(ClassType::Blacksmith),
            "ARMORER"       | "ARM" => Ok(ClassType::Armorer),
            "GOLDSMITH"     | "GSM" => Ok(ClassType::Goldsmith),
            "LEATHERWORKER" | "LTW" => Ok(ClassType::Leatherworker),
            "WEAVER"        | "WVR" => Ok(ClassType::Weaver),
            "ALCHEMIST"     | "ALC" => Ok(ClassType::Alchemist),
            "CULINARIAN"    | "CUL" => Ok(ClassType::Culinarian),
            "MINER"         | "MIN" => Ok(ClassType::Miner),
            "BOTANIST"      | "BTN" => Ok(ClassType::Botanist),
            "FISHER"        | "FSH" => Ok(ClassType::Fisher),
            x => Err(ClassTypeParseError(x.into())),
        }
    }
}

/// Holds information about a profiles level in a particular class.
#[derive(Debug, Clone)]
pub struct Classes(HashMap<ClassType, Option<u32>>);

impl Classes {
    pub fn new() -> Self {
        Classes(HashMap::new())
    }
    /// Adds or updates a given entry.
    pub fn insert(&mut self, kind: ClassType, level: Option<u32>) {
        self.0.insert(kind, level);
    }

    pub fn get(&self, class: ClassType) -> Option<u32> {
        *self.0.get(&class).unwrap_or(&None)
    }
}