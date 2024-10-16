use failure::{Error, Fail, ensure};
use select::document::Document;
use select::predicate::{Class, Name};

use std::str::FromStr;

use crate::model::{
    attribute::{Attribute, Attributes},
    gear::{Slot, EquippedGear},
    clan::Clan,
    class::{Classes, ClassInfo, ClassType},
    gender::Gender, 
    race::Race, 
    server::Server,
    datacenter::Datacenter,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct HomeInfo {
    server: Server,
    datacenter: Datacenter,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct PlayerInfo {
    class: Option<ClassType>,
    level: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FieldOps {
    bozja: Option<u32>,
    eureka: Option<u32>
}

/// Takes a Document and a search expression, and will return
/// a `SearchError` if it is not found. Otherwise it will return
/// the found node.
macro_rules! ensure_node {
    ($doc:ident, $search:expr) => {{
        ensure_node!($doc, $search, 0)
    }};
    
    ($doc:ident, $search:expr, $nth:expr) => {{
        let node = $doc.find($search).nth($nth);
        ensure!(node.is_some(), SearchError::NodeNotFound(stringify!($search).to_string() + "(" + stringify!($nth) + ")"));
        node.unwrap()
    }};
}

/// Holds all the data for a profile retrieved via Lodestone.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Profile {
    /// The id associated with the profile
    pub user_id: u32,
    /// The profile's associated Free Company
    pub free_company: Option<String>,
    /// The profile's title
    pub title: Option<String>,
    /// The character's in-game name.
    pub name: String,
    /// The character's nameday
    pub nameday: String,
    /// The character's guardian
    pub guardian: String,
    /// The character's city state
    pub city_state: String,
    /// Which server the character is in.
    pub server: Server,
    /// Which datacenter the character is in.
    pub datacenter: Datacenter,
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
    /// Current class
    pub class: Option<ClassType>,
    /// Current class's level
    pub level: u32,
    /// A list of field operation levels
    pub fieldops: FieldOps,
    /// A list of gear slots
    pub gear: EquippedGear,
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

        //  Holds the string for Server, Datacenter in that order
        let home_info = Self::parse_home_info(&main_doc)?;

        let (hp, mp) = Self::parse_char_param(&main_doc)?;

        let (player_info, gear) = Self::parse_profile_info(&main_doc)?;

        Ok(Self {
            user_id,
            free_company: Self::parse_free_company(&main_doc),
            title: Self::parse_title(&main_doc),
            name: Self::parse_name(&main_doc)?,
            nameday: Self::parse_nameday(&main_doc)?,
            guardian: Self::parse_guardian(&main_doc)?,
            city_state: Self::parse_city_state(&main_doc)?,
            server: home_info.server,
            datacenter: home_info.datacenter,
            race: char_info.race,
            clan: char_info.clan,
            gender: char_info.gender,
            hp,
            mp,
            class: player_info.class,
            level: player_info.level,
            gear,
            fieldops: Self::parse_fieldops(&classes_doc)?,
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
        match doc.find(Class("character__freecompany__name")).next() {
            Some(node) => Some(
                node.text().strip_prefix("Free Company").unwrap_or(&node.text()).to_string()
            ),
            None => None,
        }
    }

    fn parse_title(doc: &Document) -> Option<String> {
        match doc.find(Class("frame__chara__title")).next() {
            Some(node) => Some(node.text()),
            None => None,
        }
    }

    fn parse_name(doc: &Document) -> Result<String, Error> {
        Ok(ensure_node!(doc, Class("frame__chara__name")).text())
    }

    fn parse_nameday(doc: &Document) -> Result<String, Error> {
        Ok(ensure_node!(doc, Class("character-block__birth")).text())
    }

    fn parse_guardian(doc: &Document) -> Result<String, Error> {
        Ok(ensure_node!(doc, Class("character-block__name"), 1).text())
    }

    fn parse_city_state(doc: &Document) -> Result<String, Error> {
        Ok(ensure_node!(doc, Class("character-block__name"), 2).text())
    }

    fn parse_home_info(doc: &Document) -> Result<HomeInfo, Error> {
        let text = ensure_node!(doc, Class("frame__chara__world")).text();
        let mut server = text.split("\u{A0}").next();

        ensure!(server.is_some(), SearchError::InvalidData("Could not find server/datacenter string.".into()));

        // String comes in format Server [Datacenter]
        let home_info = server
            .unwrap()
            .split_whitespace()
            .map(|e| e.replace(&['[', ']'], ""))
            .collect::<Vec<String>>();
            
        Ok(HomeInfo {
            server: Server::from_str(&home_info[0])?,
            datacenter: Datacenter::from_str(&home_info[1])?,
        })
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

    fn parse_profile_info(doc: &Document) -> Result<(PlayerInfo, EquippedGear), Error> {
        let attr_block = ensure_node!(doc, Class("character__profile__detail"));
        // comes in format `LEVEL 81 `, trailing space included
        let level: u32 = ensure_node!(attr_block, Class("character__class__data"))
            .text()
            .replace("LEVEL", "")
            .replace(" ", "")
            .parse::<u32>()
            .unwrap_or(0);

        let class: Option<ClassType> = 'class: {
            // get the job icon url
            let class_icon = ensure_node!(attr_block, Class("character__class_icon"))
            .first_child()
            .unwrap()
            .attr("src");
            if class_icon.is_none() {
                break 'class None;
            }

            match ClassType::from_str(class_icon.unwrap()) {
                Ok(class_type) => Some(class_type),
                Err(_) => None,
            }
        };

        // loop through gear slots and push them to the vec
        let mut equipped_gear = Vec::with_capacity(14);
        for item in attr_block.find(Class("js__db_tooltip")) {
            // childless div = empty slot
            if item.first_child().is_none() {
                equipped_gear.push(None);
                continue;
            }

            let mut slot = Slot::default();
            slot.name = {
                match item.find(Class("db-tooltip__item__name")).next() {
                    Some(node) => Some(node.text()),
                    None => None,
                }
            };
            slot.glamour_name = {
                // using the `view item details` button on hover, and fetching the parent <p>'s text
                match item.find(Class("db-tooltip__item__mirage__btn")).next() {
                    Some(node) => Some(
                        node.parent()
                        .unwrap()
                        .text()
                    ),
                    None => None,
                }
            };
            slot.ilvl = {
                match item.find(Class("db-tooltip__item__level")).next() {
                    Some(node) => Some(
                        // comes in format `Item Level 630`
                        node.text()
                        .replace("Item Level ", "")
                        .parse::<u32>()
                        .unwrap_or(0)
                    ),
                    None => None,
                }
            };
            equipped_gear.push(Some(slot));
        };

        Ok(
            (PlayerInfo {
                class,
                level,
            },
            EquippedGear {
                mainhand:       equipped_gear[0].clone(),
                head:           equipped_gear[1].clone(),
                body:           equipped_gear[2].clone(),
                hands:          equipped_gear[3].clone(),
                legs:           equipped_gear[4].clone(),
                feet:           equipped_gear[5].clone(),
                facewear:       equipped_gear[6].clone(),
                offhand:        equipped_gear[7].clone(),
                earrings:       equipped_gear[8].clone(),
                necklace:       equipped_gear[9].clone(),
                bracelets:      equipped_gear[10].clone(),
                ring_left:      equipped_gear[11].clone(),
                ring_right:     equipped_gear[12].clone(),
                soul_crystal:   equipped_gear[13].clone(),
            }
        ))
    }

    fn parse_fieldops(doc: &Document) -> Result<FieldOps, Error> {
        let attr_block = ensure_node!(doc, Class("character__content"));

        // if not unlocked, the corresponding div is absent. otherwise its all there
        let bozja: Option<u32> = {
            match doc.find(Class("xiv-lds-resistance-level")).next() {
                Some(node) => Some(
                    node
                    .parent().unwrap()
                    .parent().unwrap()
                    .find(Class("character__job__level"))
                    .next().unwrap()
                    .text()
                    .parse::<u32>()
                    .unwrap_or(0)
                ),
                None => None,
            }
        };

        let eureka: Option<u32> = {
            match doc.find(Class("xiv-lds-elemental-level")).next() {
                Some(node) => Some(
                    node
                    .parent().unwrap()
                    .parent().unwrap()
                    .find(Class("character__job__level"))
                    .next().unwrap()
                    .text()
                    .parse::<u32>()
                    .unwrap_or(0)
                ),
                None => None,
            }
        };

        Ok(FieldOps {
            bozja,
            eureka,
        })
    }

    fn parse_char_param(doc: &Document) -> Result<(u32, u32), Error> {
        let attr_block = ensure_node!(doc, Class("character__param"));
        let mut hp = None;
        let mut mp = None;
        for item in attr_block.find(Name("li")) {
            if item.find(Class("character__param__text__hp--en-us")).count() == 1 {
                hp = Some(ensure_node!(item, Name("span")).text().parse::<u32>()?);
            } else if item.find(Class("character__param__text__mp--en-us")).count() == 1 ||
                      item.find(Class("character__param__text__gp--en-us")).count() == 1 ||
                      item.find(Class("character__param__text__cp--en-us")).count() == 1 {
                // doh/dol jobs change the css now to show GP/CP. if any is present, store as mp
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
