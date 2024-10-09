use failure::Fail;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug, Fail)]
#[fail(display = "Invalid class type '{}'", _0)]
pub struct ClassTypeParseError(String);

/// Contains all the data for a class/job insofar as it pertains to a specific character
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClassInfo {
    pub level: u32,
    pub current_xp: Option<u64>,
    pub max_xp: Option<u64>,
}

/// An enum over the types of classes or jobs that are available.
/// 
/// In the case of unlocking a job, the higher level one is preferred.
/// For example, after unlocking Paladin, the class type will return
/// Paladin instead of Gladiator.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ClassType {
    //  Tank
    Paladin,
    Gladiator,
    Warrior,
    Marauder,
    DarkKnight,
    Gunbreaker,
    //  Healer
    WhiteMage,
    Conjurer,
    Scholar,
    Astrologian,
    Sage,
    //  Melee
    Monk,
    Pugilist,
    Dragoon,
    Lancer,
    Ninja,
    Rogue,
    Samurai,
    Reaper,
    Viper,
    //   Phys Range
    Bard,
    Archer,
    Machinist,
    Dancer,
    //   Caster
    BlackMage,
    Thaumaturge,
    Summoner,
    Arcanist,
    RedMage,
    Pictomancer,
    BlueMage,
    //  DoH
    Carpenter,
    Blacksmith,
    Armorer,
    Goldsmith,
    Leatherworker,
    Weaver,
    Alchemist,
    Culinarian,
    //  DoL
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
            //   Tank
            "PALADIN"       | "PLD" => Ok(ClassType::Paladin),
            "GLADIATOR"     | "GLD" => Ok(ClassType::Gladiator),
            "WARRIOR"       | "WAR" => Ok(ClassType::Warrior),
            "MARAUDER"      | "MRD" => Ok(ClassType::Marauder),
            "DARK KNIGHT"   | "DRK" => Ok(ClassType::DarkKnight),
            "GUNBREAKER"    | "GNB" => Ok(ClassType::Gunbreaker),
            //   Healer
            "WHITE MAGE"    | "WHM" => Ok(ClassType::WhiteMage),
            "CONJURER"      | "CNJ" => Ok(ClassType::Conjurer),
            "SCHOLAR"       | "SCH" => Ok(ClassType::Scholar),
            "ASTROLOGIAN"   | "AST" => Ok(ClassType::Astrologian),
            "SAGE"          | "SGE" => Ok(ClassType::Sage),
            //   Melee
            "MONK"          | "MNK" => Ok(ClassType::Monk),
            "PUGILIST"      | "PUG" => Ok(ClassType::Pugilist),
            "DRAGOON"       | "DRG" => Ok(ClassType::Dragoon),
            "LANCER"        | "LNC" => Ok(ClassType::Lancer),
            "NINJA"         | "NIN" => Ok(ClassType::Ninja),
            "ROGUE"         | "ROG" => Ok(ClassType::Rogue),
            "SAMURAI"       | "RPR" => Ok(ClassType::Samurai),
            "REAPER"        | "SAM" => Ok(ClassType::Reaper),
            "VIPER"         | "VPR" => Ok(ClassType::Viper),
            //   Phys Range
            "BARD"          | "BRD" => Ok(ClassType::Bard),
            "ARCHER"        | "ARC" => Ok(ClassType::Archer),
            "MACHINIST"     | "MCH" => Ok(ClassType::Machinist),
            "DANCER"        | "DNC" => Ok(ClassType::Dancer),
            //   Caster
            "BLACK MAGE"    | "BLM" => Ok(ClassType::BlackMage),
            "THAUMATURGE"   | "THM" => Ok(ClassType::Thaumaturge),
            "SUMMONER"      | "SMN" => Ok(ClassType::Summoner),
            "ARCANIST"      | "ACN" => Ok(ClassType::Arcanist),
            "RED MAGE"      | "RDM" => Ok(ClassType::RedMage),
            "PICTOMANCER"   | "PCT" => Ok(ClassType::Pictomancer),
            "BLUE MAGE" | "BLUE MAGE (LIMITED JOB)" | "BLU" => Ok(ClassType::BlueMage),
            //   DoH
            "CARPENTER"     | "CRP" => Ok(ClassType::Carpenter),
            "BLACKSMITH"    | "BSM" => Ok(ClassType::Blacksmith),
            "ARMORER"       | "ARM" => Ok(ClassType::Armorer),
            "GOLDSMITH"     | "GSM" => Ok(ClassType::Goldsmith),
            "LEATHERWORKER" | "LTW" => Ok(ClassType::Leatherworker),
            "WEAVER"        | "WVR" => Ok(ClassType::Weaver),
            "ALCHEMIST"     | "ALC" => Ok(ClassType::Alchemist),
            "CULINARIAN"    | "CUL" => Ok(ClassType::Culinarian),
            //   DoL
            "MINER"         | "MIN" => Ok(ClassType::Miner),
            "BOTANIST"      | "BTN" => Ok(ClassType::Botanist),
            "FISHER"        | "FSH" => Ok(ClassType::Fisher),
            x => Err(ClassTypeParseError(x.into())),
        }
    }
}

/// Holds information about a profile's level/XP in a particular class.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Classes(HashMap<ClassType, Option<ClassInfo>>);

impl Classes {
    pub fn new() -> Self {
        Classes(HashMap::new())
    }
    /// Adds or updates a given entry.
    pub fn insert(&mut self, kind: ClassType, class: Option<ClassInfo>) {
        self.0.insert(kind, class);
    }

    /// Gets a class by name, if found
    pub fn get(&self, class: ClassType) -> Option<ClassInfo> {
        *self.0.get(&class).unwrap_or(&None)
    }
}
