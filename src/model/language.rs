use failure::Fail;
use std::str::FromStr;

#[derive(Clone, Debug, Fail)]
#[fail(display = "Invalid language string '{}'", _0)]
pub struct LanguageParseError(String);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Language {
    Japanese,
    English,
    German,
    French,
}

impl FromStr for Language {
    type Err = LanguageParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_uppercase() {
            "JAPANESE" | "JA" => Ok(Language::Japanese),
            "ENGLISH" | "EN" => Ok(Language::English),
            "GERMAN" | "DE" => Ok(Language::German),
            "FRENCH" | "FR" => Ok(Language::French),
            x => Err(LanguageParseError(x.into())),
        }
    }
}