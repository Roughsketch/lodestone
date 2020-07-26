use failure::{Error, Fail, ensure};
use select::document::Document;
use select::predicate::{Class, Name};

use std::str::FromStr;

use crate::model::{
    attribute::{Attribute, Attributes},
    clan::Clan,
    class::{Classes, ClassInfo, ClassType},
    gender::Gender, 
    race::Race, 
    server::Server,
    util::load_url
};

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
    /// Max HP.
    pub hp: u32,
    /// Max MP.
    pub mp: u32,
    /// A list of attributes and their values.
    pub attributes: Attributes,
    /// A list of classes and their corresponding levels.
    classes: Classes,
}

impl Profile {
    /// Gets a profile for a user given their lodestone user id.
    /// 
    /// If you don't have the id, it is possible to use a 
    /// `SearchBuilder` in order to find their profile directly.
    pub fn get(user_id: u32) -> Result<Self, Error> {
        let main_doc = load_url(user_id, None)?;
        let classes_doc = load_url(user_id, Some("class_job"))?;

        //  Holds the string for Race, Clan, and Gender in that order
        let char_info = Self::parse_char_info(&main_doc)?;
        let (hp, mp) = Self::parse_char_param(&main_doc)?;

        Ok(Self {
            user_id,
            free_company: Self::parse_free_company(&main_doc),
            name: Self::parse_name(&main_doc)?,
            server: Self::parse_server(&main_doc)?,
            race: char_info.race,
            clan: char_info.clan,
            gender: char_info.gender,
            hp,
            mp,
            attributes: Self::parse_attributes(&main_doc)?,
            classes: Self::parse_classes(&classes_doc)?,
        })
    }

    /// Get the level of a specific class for this profile.
    /// 
    /// This can be used to query whether or not a job is unlocked.
    /// For instance if Gladiator is below 30, then Paladin will 
    /// return None. If Paladin is unlocked, both Gladiator and
    /// Paladin will return the same level.
    pub fn level(&self, class: ClassType) -> Option<u32> {
        match self.class_info(class) {
            Some(v) => Some(v.level),
            None => None
        }
    }

    /// Gets this profile's data for a given class
    pub fn class_info(&self, class: ClassType) -> Option<ClassInfo> {
        self.classes.get(class)
    }

    /// Borrows the full map of classes, e.g. for iteration in calling code
    pub fn all_class_info(&self) -> &Classes {
        &self.classes
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

    fn parse_char_param(doc: &Document) -> Result<(u32, u32), Error> {
        let attr_block = ensure_node!(doc, Class("character__param"));
        let mut hp = None;
        let mut mp = None;
        for item in attr_block.find(Name("li")) {
            if item.find(Class("character__param__text__hp--en-us")).count() == 1 {
                hp = Some(ensure_node!(item, Name("span")).text().parse::<u32>()?);
            } else if item.find(Class("character__param__text__mp--en-us")).count() == 1 {
                mp = Some(ensure_node!(item, Name("span")).text().parse::<u32>()?);
            } else {
                continue
            }
        }
        ensure!(hp.is_some() && mp.is_some(), SearchError::InvalidData("character__param".into()));
        Ok((hp.unwrap(), mp.unwrap()))
    }

    fn parse_attributes(doc: &Document) -> Result<Attributes, Error> {
        let block = ensure_node!(doc, Class("character__profile__data"));
        let mut attributes = Attributes::new();
        for item in block.find(Name("tr")) {
            let name = ensure_node!(item, Name("span")).text();
            let value = Attribute{
                level: ensure_node!(item, Name("td")).text().parse::<u16>()?
            };
            attributes.insert(name, value);
        }
        Ok(attributes)
    }

    fn parse_classes(doc: &Document) -> Result<Classes, Error> {
        let mut classes = Classes::new();

        for list in doc.find(Class("character__content")).take(4) {
            for item in list.find(Name("li")) {
                let name = ensure_node!(item, Class("character__job__name")).text();
                let classinfo = match ensure_node!(item, Class("character__job__level")).text().as_str() {
                    "-" => None,
                    level => {
                        let text = ensure_node!(item, Class("character__job__exp")).text();
                        let mut parts = text.split(" / ");
                        let current_xp = parts.next();
                        ensure!(current_xp.is_some(), SearchError::InvalidData("character__job__exp".into()));
                        let max_xp = parts.next();
                        ensure!(max_xp.is_some(), SearchError::InvalidData("character__job__exp".into()));
                        Some(ClassInfo{
                            level: level.parse()?,
                            current_xp: match current_xp.unwrap() {
                                "--" => None,
                                value => Some(value.replace(",", "").parse()?)
                            },
                            max_xp: match max_xp.unwrap() {
                                "--" => None,
                                value => Some(value.replace(",", "").parse()?)
                            },
                        })
                    }
                };

                //  For classes that have multiple titles (e.g., Paladin / Gladiator), grab the first one.
                let name = name.split(" / ").next();
                ensure!(name.is_some(), SearchError::InvalidData("character__job__name".into()));
                let class = ClassType::from_str(&name.unwrap())?;

                //  If the class added was a secondary job, then associated that level
                //  with its lower level counterpart as well. This makes returning the
                //  level for a particular grouping easier at the cost of memory.
                match class {
                    ClassType::Paladin => classes.insert(ClassType::Gladiator, classinfo),
                    ClassType::Warrior => classes.insert(ClassType::Marauder, classinfo),
                    ClassType::WhiteMage => classes.insert(ClassType::Conjurer, classinfo),
                    ClassType::Monk => classes.insert(ClassType::Pugilist, classinfo),
                    ClassType::Dragoon => classes.insert(ClassType::Lancer, classinfo),
                    ClassType::Ninja => classes.insert(ClassType::Rogue, classinfo),
                    ClassType::Bard => classes.insert(ClassType::Archer, classinfo),
                    ClassType::BlackMage => classes.insert(ClassType::Thaumaturge, classinfo),
                    ClassType::Summoner => classes.insert(ClassType::Arcanist, classinfo),
                    _ => (),
                }

                classes.insert(class, classinfo);
            }
        }

        Ok(classes)
    }
}
