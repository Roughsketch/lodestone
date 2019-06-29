use failure::Fail;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Fail)]
#[fail(display = "Invalid datacenter string '{}'", _0)]
pub struct DatacenterParseError(String);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Datacenter {
    Aether,
    Chaos,
    Crystal,
    Elemental,
    Gaia,
    Mana,
    Primal,
}

/// Case insensitive FromStr impl for datacenters.
impl FromStr for Datacenter {
    type Err = DatacenterParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_uppercase() {
            "AETHER" => Ok(Datacenter::Aether),
            "CHAOS" => Ok(Datacenter::Chaos),
            "CRYSTAL" => Ok(Datacenter::Crystal),
            "ELEMENTAL" => Ok(Datacenter::Elemental),
            "GAIA" => Ok(Datacenter::Gaia),
            "MANA" => Ok(Datacenter::Mana),
            "PRIMAL" => Ok(Datacenter::Primal),
            x => Err(DatacenterParseError(x.into())),
        }
    }
}

impl fmt::Display for Datacenter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let datacenter = match *self {
            Datacenter::Aether => "Aether",
            Datacenter::Chaos => "Chaos",
            Datacenter::Crystal => "Crystal",
            Datacenter::Elemental => "Elemental",
            Datacenter::Gaia => "Gaia",
            Datacenter::Mana => "Mana",
            Datacenter::Primal => "Primal",
        };

        write!(f, "{}", datacenter)
    }
}