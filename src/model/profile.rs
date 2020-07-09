use failure::{Error, Fail, ensure};
use select::document::Document;
use select::predicate::{Class, Name};

use crate::CLIENT;

use std::str::FromStr;

use crate::model::{
    clan::Clan,
    class::{Classes, ClassType},
    gender::Gender, 
    race::Race, 
    server::Server
};

/// The URL base for profiles.
static BASE_PROFILE_URL: &str = "https://na.finalfantasyxiv.com/lodestone/character/";

/// Represents ways in which a search over the HTML data might go wrong.
#[derive(Fail, Debug)]
pub enum SearchError {
    /// A search for a node that was required turned up empty.
    #[fail(display = "Node not found: {}", _0)]
    NodeNotFound(String),
    /// A node was found, but the data inside it was malformed.
    #[fail(display = "Invalid data found while parsing '{}'", _0)]
    InvalidData(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct CharInfo {
    race: Race,
    clan: Clan,
    gender: Gender,
}

/// Takes a Document and a search expression, and will return
/// a `SearchError` if it is not found. Otherwise it will return
/// the found node.
macro_rules! ensure_node {
    ($doc:ident, $search:expr) => {{
        let node = $doc.find($search).next();
        ensure!(node.is_some(), SearchError::NodeNotFound(stringify!($search).to_string()));
        node.unwrap()
    }}
}

/// Holds all the data for a profile retrieved via Lodestone.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Profile {
    /// The id associated with the profile
    pub user_id: u32,
    /// The profile's associated Free Company
    pub free_company: Option<String>,
    /// The character's in-game name.
    pub name: String,
    /// Which server the character is in.
    pub server: Server,
    /// What race the character is.
    pub race: Race,
    /// One of the two clans associated with their race.
    pub clan: Clan,
    /// Character's gender.
    pub gender: Gender,
    /// A list of classes and their corresponding levels.
    classes: Classes,
}

impl Profile {
    /// Gets a profile for a user given their lodestone user id.
    /// 
    /// If you don't have the id, it is possible to use a 
    /// `SearchBuilder` in order to find their profile directly.
    pub fn get(user_id: u32) -> Result<Self, Error> {
        let mut response = CLIENT.get(&format!("{}{}/", BASE_PROFILE_URL, user_id)).send()?;
        let text = response.text()?;
        let doc = Document::from(text.as_str());

        //  Holds the string for Race, Clan, and Gender in that order
        let char_info = Self::parse_char_info(&doc)?;

        Ok(Self {
            user_id,
            free_company: Self::parse_free_company(&doc),
            name: Self::parse_name(&doc)?,
            server: Self::parse_server(&doc)?,
            race: char_info.race,
            clan: char_info.clan,
            gender: char_info.gender,
            classes: Self::parse_classes(&doc)?,
        })
    }

    /// Get the level of a specific class for this profile.
    /// 
    /// This can be used to query whether or not a job is unlocked.
    /// For instance if Gladiator is below 30, then Paladin will 
    /// return None. If Paladin is unlocked, both Gladiator and
    /// Paladin will return the same level.
    pub fn level(&self, class: ClassType) -> Option<u32> {
        self.classes.get(class)
    }

    fn parse_free_company(doc: &Document) -> Option<String> {
        match doc.find(Class("frame__chara__title")).next() {
            Some(node) => Some(node.text()),
            None => None,
        }
    }

    fn parse_name(doc: &Document) -> Result<String, Error> {
        Ok(ensure_node!(doc, Class("frame__chara__name")).text())
    }

    fn parse_server(doc: &Document) -> Result<Server, Error> {
        let text = ensure_node!(doc, Class("frame__chara__world")).text();
        let server = text.split("\u{A0}").next();

        ensure!(server.is_some(), SearchError::InvalidData("Could not find server string.".into()));

        Ok(Server::from_str(&server.unwrap())?)
    }

    fn parse_char_info(doc: &Document) -> Result<CharInfo, Error> {
        let char_block = {
            let mut block = ensure_node!(doc, Class("character-block__name")).inner_html();
            block = block.replace(" ", "_");
            block = block.replace("<br>", " ");
            block.replace("_/_", " ")
        };

        let char_info = char_block
            .split_whitespace()
            .map(|e| e.replace("_", " "))
            .map(|e| e.into())
            .collect::<Vec<String>>();

        ensure!(char_info.len() == 3 || char_info.len() == 4, SearchError::InvalidData("character block name".into()));

        //  If the length is 4, then the race is "Au Ra"
        if char_info.len() == 4 {
            Ok(CharInfo {
                race: Race::Aura,
                clan: Clan::from_str(&char_info[2])?,
                gender: Gender::from_str(&char_info[3])?,
            })
        } else {
            Ok(CharInfo {
                race: Race::from_str(&char_info[0])?,
                clan: Clan::from_str(&char_info[1])?,
                gender: Gender::from_str(&char_info[2])?,
            })
        }
    }

    fn parse_classes(doc: &Document) -> Result<Classes, Error> {
        let mut classes = Classes::new();

        for list in doc.find(Class("character__level__list")).take(4) {
            for item in list.find(Name("li")) {
                let text = ensure_node!(item, Name("img")).attr("data-tooltip");
                let level = match &*item.text() {
                    "-" => None,
                    num => Some(num.parse::<u32>()?),
                };

                ensure!(text.is_some(), SearchError::InvalidData("data-tooltip".into()));

                //  For classes that have multiple titles (e.g., Paladin / Gladiator), grab the first one.
                let name = text.unwrap().split(" / ").next();

                ensure!(name.is_some(), SearchError::InvalidData("data-tooltip".into()));

                let class = ClassType::from_str(name.unwrap())?;

                //  If the class added was a secondary job, then associated that level
                //  with its lower level counterpart as well. This makes returning the
                //  level for a particular grouping easier at the cost of memory.
                match class {
                    ClassType::Paladin => classes.insert(ClassType::Gladiator, level),
                    ClassType::Warrior => classes.insert(ClassType::Marauder, level),
                    ClassType::WhiteMage => classes.insert(ClassType::Conjurer, level),
                    ClassType::Monk => classes.insert(ClassType::Pugilist, level),
                    ClassType::Dragoon => classes.insert(ClassType::Lancer, level),
                    ClassType::Ninja => classes.insert(ClassType::Rogue, level),
                    ClassType::Bard => classes.insert(ClassType::Archer, level),
                    ClassType::BlackMage => classes.insert(ClassType::Thaumaturge, level),
                    ClassType::Summoner => classes.insert(ClassType::Arcanist, level),
                    _ => (),
                }

                classes.insert(class, level);
            }
        }

        Ok(classes)
    }
}
