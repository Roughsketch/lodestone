use failure::Fail;
use std::str::FromStr;

#[derive(Clone, Debug, Fail)]
#[fail(display = "Invalid gender string '{}'", _0)]
pub struct GenderParseError(String);

/// Enumeration for the gender of a character.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Gender {
    Female,
    Male,
}

impl FromStr for Gender {
    type Err = GenderParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "♀" => Ok(Gender::Female),
            "♂" => Ok(Gender::Male),
            x => Err(GenderParseError(x.into())),
        }
    }
}