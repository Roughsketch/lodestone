use failure::Fail;
use std::str::FromStr;

#[derive(Clone, Debug, Fail)]
#[fail(display = "Invalid clan string '{}'", _0)]
pub struct ClanParseError(String);

/// Enumeration for the clans available in XIV.
#[derive(Clone, Debug, PartialEq)]
pub enum Clan {
    //  Au Ra
    Xaela,
    Raen,
    //  Elezen
    Wildwood,
    Duskwight,
    //  Hyur
    Midlander,
    Highlander,
    //  Lalafell
    Dunesfolk,
    Plainsfork,
    //  Miqo'te
    SeekerOfTheSun,
    KeeperOfTheMoon,
    //  Roegadyn
    SeaWolf,
    Hellsguard,
}

impl FromStr for Clan {
    type Err = ClanParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_uppercase() {
            "XAELA" => Ok(Clan::Xaela),
            "RAEN" => Ok(Clan::Raen),
            "WILDWOOD" => Ok(Clan::Wildwood),
            "DUSKWIGHT" => Ok(Clan::Duskwight),
            "MIDLANDER" => Ok(Clan::Midlander),
            "HIGHLANDER" => Ok(Clan::Highlander),
            "DUNESFOLK" => Ok(Clan::Dunesfolk),
            "PLAINSFORK" => Ok(Clan::Plainsfork),
            "SEEKER OF THE SUN" => Ok(Clan::SeekerOfTheSun),
            "KEEPER OF THE MOON" => Ok(Clan::KeeperOfTheMoon),
            "SEA WOLF" => Ok(Clan::SeaWolf),
            "HELLSGUARD" => Ok(Clan::Hellsguard),
            x => Err(ClanParseError(x.into())),
        }
    }
}