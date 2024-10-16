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
        // if string starts as a url, just leave it be
        let search = match s.starts_with("https://") {
            true => s,
            false => &*s.to_uppercase(),
        };
        match search {
            //   Tank
            "PALADIN"       | "PLD" | "https://lds-img.finalfantasyxiv.com/h/E/d0Tx-vhnsMYfYpGe9MvslemEfg.png" => Ok(ClassType::Paladin),
            "GLADIATOR"     | "GLD" | "https://lds-img.finalfantasyxiv.com/h/U/F5JzG9RPIKFSogtaKNBk455aYA.png" => Ok(ClassType::Gladiator),
            "WARRIOR"       | "WAR" | "https://lds-img.finalfantasyxiv.com/h/y/A3UhbjZvDeN3tf_6nJ85VP0RY0.png" => Ok(ClassType::Warrior),
            "MARAUDER"      | "MRD" | "https://lds-img.finalfantasyxiv.com/h/N/St9rjDJB3xNKGYg-vwooZ4j6CM.png" => Ok(ClassType::Marauder),
            "DARK KNIGHT"   | "DRK" | "https://lds-img.finalfantasyxiv.com/h/l/5CZEvDOMYMyVn2td9LZigsgw9s.png" => Ok(ClassType::DarkKnight),
            "GUNBREAKER"    | "GNB" | "https://lds-img.finalfantasyxiv.com/h/8/hg8ofSSOKzqng290No55trV4mI.png" => Ok(ClassType::Gunbreaker),
            //   Healer
            "WHITE MAGE"    | "WHM" | "https://lds-img.finalfantasyxiv.com/h/7/i20QvSPcSQTybykLZDbQCgPwMw.png" => Ok(ClassType::WhiteMage),
            "CONJURER"      | "CNJ" | "https://lds-img.finalfantasyxiv.com/h/s/gl62VOTBJrm7D_BmAZITngUEM8.png" => Ok(ClassType::Conjurer),
            "SCHOLAR"       | "SCH" | "https://lds-img.finalfantasyxiv.com/h/7/WdFey0jyHn9Nnt1Qnm-J3yTg5s.png" => Ok(ClassType::Scholar),
            "ASTROLOGIAN"   | "AST" | "https://lds-img.finalfantasyxiv.com/h/1/erCgjnMSiab4LiHpWxVc-tXAqk.png" => Ok(ClassType::Astrologian),
            "SAGE"          | "SGE" | "https://lds-img.finalfantasyxiv.com/h/g/_oYApASVVReLLmsokuCJGkEpk0.png" => Ok(ClassType::Sage),
            //   Melee
            "MONK"          | "MNK" | "https://lds-img.finalfantasyxiv.com/h/K/HW6tKOg4SOJbL8Z20GnsAWNjjM.png" => Ok(ClassType::Monk),
            "PUGILIST"      | "PUG" | "https://lds-img.finalfantasyxiv.com/h/V/iW7IBKQ7oglB9jmbn6LwdZXkWw.png" => Ok(ClassType::Pugilist),
            "DRAGOON"       | "DRG" | "https://lds-img.finalfantasyxiv.com/h/m/gX4OgBIHw68UcMU79P7LYCpldA.png" => Ok(ClassType::Dragoon),
            "LANCER"        | "LNC" | "https://lds-img.finalfantasyxiv.com/h/k/tYTpoSwFLuGYGDJMff8GEFuDQs.png" => Ok(ClassType::Lancer),
            "NINJA"         | "NIN" | "https://lds-img.finalfantasyxiv.com/h/0/Fso5hanZVEEAaZ7OGWJsXpf3jw.png" => Ok(ClassType::Ninja),
            "ROGUE"         | "ROG" | "https://lds-img.finalfantasyxiv.com/h/y/wdwVVcptybfgSruoh8R344y_GA.png" => Ok(ClassType::Rogue),
            "SAMURAI"       | "SAM" | "https://lds-img.finalfantasyxiv.com/h/m/KndG72XtCFwaq1I1iqwcmO_0zc.png" => Ok(ClassType::Samurai),
            "REAPER"        | "RPR" | "https://lds-img.finalfantasyxiv.com/h/7/cLlXUaeMPJDM2nBhIeM-uDmPzM.png" => Ok(ClassType::Reaper),
            "VIPER"         | "VPR" | "https://lds-img.finalfantasyxiv.com/h/C/WojNTqMJ_Ye1twvkIhw825zc20.png" => Ok(ClassType::Viper),
            //   Phys Range
            "BARD"          | "BRD" | "https://lds-img.finalfantasyxiv.com/h/F/KWI-9P3RX_Ojjn_mwCS2N0-3TI.png" => Ok(ClassType::Bard),
            "ARCHER"        | "ARC" | "https://lds-img.finalfantasyxiv.com/h/Q/ZpqEJWYHj9SvHGuV9cIyRNnIkk.png" => Ok(ClassType::Archer),
            "MACHINIST"     | "MCH" | "https://lds-img.finalfantasyxiv.com/h/E/vmtbIlf6Uv8rVp2YFCWA25X0dc.png" => Ok(ClassType::Machinist),
            "DANCER"        | "DNC" | "https://lds-img.finalfantasyxiv.com/h/t/HK0jQ1y7YV9qm30cxGOVev6Cck.png" => Ok(ClassType::Dancer),
            //   Caster
            "BLACK MAGE"    | "BLM" | "https://lds-img.finalfantasyxiv.com/h/P/V01m8YRBYcIs5vgbRtpDiqltSE.png" => Ok(ClassType::BlackMage),
            "THAUMATURGE"   | "THM" | "https://lds-img.finalfantasyxiv.com/h/4/IM3PoP6p06GqEyReygdhZNh7fU.png" => Ok(ClassType::Thaumaturge),
            "SUMMONER"      | "SMN" | "https://lds-img.finalfantasyxiv.com/h/h/4ghjpyyuNelzw1Bl0sM_PBA_FE.png" => Ok(ClassType::Summoner),
            "ARCANIST"      | "ACN" | "https://lds-img.finalfantasyxiv.com/h/e/VYP1LKTDpt8uJVvUT7OKrXNL9E.png" => Ok(ClassType::Arcanist),
            "RED MAGE"      | "RDM" | "https://lds-img.finalfantasyxiv.com/h/q/s3MlLUKmRAHy0pH57PnFStHmIw.png" => Ok(ClassType::RedMage),
            "PICTOMANCER"   | "PCT" | "https://lds-img.finalfantasyxiv.com/h/_/kLob-U-yh652LQPX1NHpLlUYQY.png" => Ok(ClassType::Pictomancer),
            "BLUE MAGE" | "BLUE MAGE (LIMITED JOB)" | "BLU" | "https://lds-img.finalfantasyxiv.com/h/p/jdV3RRKtWzgo226CC09vjen5sk.png" => Ok(ClassType::BlueMage),
            //   DoH
            "CARPENTER"     | "CRP" | "https://lds-img.finalfantasyxiv.com/h/v/YCN6F-xiXf03Ts3pXoBihh2OBk.png" => Ok(ClassType::Carpenter),
            "BLACKSMITH"    | "BSM" | "https://lds-img.finalfantasyxiv.com/h/5/EEHVV5cIPkOZ6v5ALaoN5XSVRU.png" => Ok(ClassType::Blacksmith),
            "ARMORER"       | "ARM" | "https://lds-img.finalfantasyxiv.com/h/G/Rq5wcK3IPEaAB8N-T9l6tBPxCY.png" => Ok(ClassType::Armorer),
            "GOLDSMITH"     | "GSM" | "https://lds-img.finalfantasyxiv.com/h/L/LbEjgw0cwO_2gQSmhta9z03pjM.png" => Ok(ClassType::Goldsmith),
            "LEATHERWORKER" | "LTW" | "https://lds-img.finalfantasyxiv.com/h/b/ACAcQe3hWFxbWRVPqxKj_MzDiY.png" => Ok(ClassType::Leatherworker),
            "WEAVER"        | "WVR" | "https://lds-img.finalfantasyxiv.com/h/X/E69jrsOMGFvFpCX87F5wqgT_Vo.png" => Ok(ClassType::Weaver),
            "ALCHEMIST"     | "ALC" | "https://lds-img.finalfantasyxiv.com/h/C/bBVQ9IFeXqjEdpuIxmKvSkqalE.png" => Ok(ClassType::Alchemist),
            "CULINARIAN"    | "CUL" | "https://lds-img.finalfantasyxiv.com/h/m/1kMI2v_KEVgo30RFvdFCyySkFo.png" => Ok(ClassType::Culinarian),
            //   DoL
            "MINER"         | "MIN" | "https://lds-img.finalfantasyxiv.com/h/A/aM2Dd6Vo4HW_UGasK7tLuZ6fu4.png" => Ok(ClassType::Miner),
            "BOTANIST"      | "BTN" | "https://lds-img.finalfantasyxiv.com/h/I/jGRnjIlwWridqM-mIPNew6bhHM.png" => Ok(ClassType::Botanist),
            "FISHER"        | "FSH" | "https://lds-img.finalfantasyxiv.com/h/x/B4Azydbn7Prubxt7OL9p1LZXZ0.png" => Ok(ClassType::Fisher),
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
