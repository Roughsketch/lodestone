use failure::Fail;
use std::str::FromStr;

#[derive(Clone, Debug, Fail)]
#[fail(display = "Invalid race string '{}'", _0)]
pub struct RaceParseError(String);

/// Models the races available in XIV.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Race {
    Aura,
    Elezen,
    Hyur,
    Lalafell,
    Miqote,
    Roegadyn,
}

impl FromStr for Race {
    type Err = RaceParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_uppercase() {
            "AU RA" => Ok(Race::Aura),
            "ELEZEN" => Ok(Race::Elezen),
            "HYUR" => Ok(Race::Hyur),
            "LALAFELL" => Ok(Race::Lalafell),
            "MIQO'TE" => Ok(Race::Miqote),
            "ROEGADYN" => Ok(Race::Roegadyn),
            x => Err(RaceParseError(x.into())),
        }
    }
}