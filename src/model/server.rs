use failure::Fail;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Fail)]
#[fail(display = "Invalid server string '{}'", _0)]
pub struct ServerParseError(String);

/// An enumeration for the servers that are currently available.
/// This list is taken from https://na.finalfantasyxiv.com/lodestone/worldstatus/
/// and the order should be identical.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Server {
    /// NA
    //  Aether
    Adamantoise,
    Cactuar,
    Faerie,
    Gilgamesh,
    Jenova,
    Midgardsormr,
    Sargatanas,
    Siren,

    //  Primal
    Behemoth,
    Excalibur,
    Exodus,
    Famfrit,
    Hyperion,
    Lamia,
    Leviathan,
    Ultros,

    // Crystal
    Balmung,
    Brynhildr,
    Coeurl,
    Diabolos,
    Goblin,
    Malboro,
    Mateus,
    Zalera,
    
    // Dynamis
    Halicarnassus,
    Maduin,
    Marilith,
    Seraph,
    Cuchulainn,
    Kraken,
    Rafflesia,
    Golem,

    /// EU
    //  Chaos
    Cerberus,
    Louisoix,
    Moogle,
    Omega,
    Phantom,
    Ragnarok,
    Sagittarius,
    Spriggan,

    //  Light
    Alpha,
    Lich,
    Odin,
    Phoenix,
    Raiden,
    Shiva,
    Twintania,
    Zodiark,

    //  Shadow (Temp DC for dawntrail launch)
    Innocence,
    Pixie,
    Titania,
    Tycoon,

    /// OCE
    //  Materia
    Bismarck,
    Ravana,
    Sephirot,
    Sophia,
    Zurvan,

    /// JP
    //  Elemental
    Aegis,
    Atomos,
    Carbuncle,
    Garuda,
    Gungnir,
    Kujata,
    Tonberry,
    Typhon,

    //  Gaia
    Alexander,
    Bahamut,
    Durandal,
    Fenrir,
    Ifrit,
    Ridill,
    Tiamat,
    Ultima,

    //  Mana
    Anima,
    Asura,
    Chocobo,
    Hades,
    Ixion,
    Masamune,
    Pandaemonium,
    Titan,

    //  Meteor
    Belias,
    Mandragora,
    Ramuh,
    Shinryu,
    Unicorn,
    Valefor,
    Yojimbo,
    Zeromus,
}

/// Case insensitive FromStr impl for servers.
impl FromStr for Server {
    type Err = ServerParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

       let fixed_world_name: &str = &*s.split_whitespace().next().unwrap();
        match &*fixed_world_name.to_uppercase() {
            
            /// NA
            //  Aether
            "ADAMANTOISE" => Ok(Server::Adamantoise),
            "CACTUAR" => Ok(Server::Cactuar),
            "FAERIE" => Ok(Server::Faerie),
            "GILGAMESH" => Ok(Server::Gilgamesh),
            "JENOVA" => Ok(Server::Jenova),
            "MIDGARDSORMR" => Ok(Server::Midgardsormr),
            "SARGATANAS" => Ok(Server::Sargatanas),
            "SIREN" => Ok(Server::Siren),

            //  Primal
            "BEHEMOTH" => Ok(Server::Behemoth),
            "EXCALIBUR" => Ok(Server::Excalibur),
            "EXODUS" => Ok(Server::Exodus),
            "FAMFRIT" => Ok(Server::Famfrit),
            "HYPERION" => Ok(Server::Hyperion),
            "LAMIA" => Ok(Server::Lamia),
            "LEVIATHAN" => Ok(Server::Leviathan),
            "ULTROS" => Ok(Server::Ultros),

            // Crystal
            "BALMUNG" => Ok(Server::Balmung),
            "BRYNHILDR" => Ok(Server::Brynhildr),
            "COEURL" => Ok(Server::Coeurl),
            "DIABOLOS" => Ok(Server::Diabolos),
            "GOBLIN" => Ok(Server::Goblin),
            "MALBORO" => Ok(Server::Malboro),
            "MATEUS" => Ok(Server::Mateus),
            "ZALERA" => Ok(Server::Zalera),
            
            // Dynamis
            "HALICARNASSUS" => Ok(Server::Halicarnassus),
            "MADUIN" => Ok(Server::Maduin),
            "MARILITH" => Ok(Server::Marilith),
            "SERAPH" => Ok(Server::Seraph),
            "CUCHULAINN" => Ok(Server::Cuchulainn),
            "KRAKEN" => Ok(Server::Kraken),
            "RAFFLESIA" => Ok(Server::Rafflesia),
            "GOLEM" => Ok(Server::Golem),

            /// EU
            //  Chaos
            "CERBERUS" => Ok(Server::Cerberus),
            "LOUISOIX" => Ok(Server::Louisoix),
            "MOOGLE" => Ok(Server::Moogle),
            "OMEGA" => Ok(Server::Omega),
            "PHANTOM" => Ok(Server::Phantom),
            "RAGNAROK" => Ok(Server::Ragnarok),
            "SAGITTARIUS" => Ok(Server::Sagittarius),
            "SPRIGGAN" => Ok(Server::Spriggan),

            //  Light
            "ALPHA" => Ok(Server::Alpha),
            "LICH" => Ok(Server::Lich),
            "ODIN" => Ok(Server::Odin),
            "PHOENIX" => Ok(Server::Phoenix),
            "RAIDEN" => Ok(Server::Raiden),
            "SHIVA" => Ok(Server::Shiva),
            "TWINTANIA" => Ok(Server::Twintania),
            "ZODIARK" => Ok(Server::Zodiark),

            //  Shadow (Temp DC for dawntrail launch)
            "INNOCENCE" => Ok(Server::Innocence),
            "PIXIE" => Ok(Server::Pixie),
            "TITANIA" => Ok(Server::Titania),
            "TYCOON" => Ok(Server::Tycoon),

            /// OCE
            //  Materia
            "BISMARCK" => Ok(Server::Bismarck),
            "RAVANA" => Ok(Server::Ravana),
            "SEPHIROT" => Ok(Server::Sephirot),
            "SOPHIA" => Ok(Server::Sophia),
            "ZURVAN" => Ok(Server::Zurvan),

            /// JP
            //  Elemental
            "AEGIS" => Ok(Server::Aegis),
            "ATOMOS" => Ok(Server::Atomos),
            "CARBUNCLE" => Ok(Server::Carbuncle),
            "GARUDA" => Ok(Server::Garuda),
            "GUNGNIR" => Ok(Server::Gungnir),
            "KUJATA" => Ok(Server::Kujata),
            "TONBERRY" => Ok(Server::Tonberry),
            "TYPHON" => Ok(Server::Typhon),

            //  Gaia
            "ALEXANDER" => Ok(Server::Alexander),
            "BAHAMUT" => Ok(Server::Bahamut),
            "DURANDAL" => Ok(Server::Durandal),
            "FENRIR" => Ok(Server::Fenrir),
            "IFRIT" => Ok(Server::Ifrit),
            "RIDILL" => Ok(Server::Ridill),
            "TIAMAT" => Ok(Server::Tiamat),
            "ULTIMA" => Ok(Server::Ultima),

            //  Mana
            "ANIMA" => Ok(Server::Anima),
            "ASURA" => Ok(Server::Asura),
            "CHOCOBO" => Ok(Server::Chocobo),
            "HADES" => Ok(Server::Hades),
            "IXION" => Ok(Server::Ixion),
            "MASAMUNE" => Ok(Server::Masamune),
            "PANDAEMONIUM" => Ok(Server::Pandaemonium),
            "TITAN" => Ok(Server::Titan),

            //  Meteor
            "BELIAS" => Ok(Server::Belias),
            "MANDRAGORA" => Ok(Server::Mandragora),
            "RAMUH" => Ok(Server::Ramuh),
            "SHINRYU" => Ok(Server::Shinryu),
            "UNICORN" => Ok(Server::Unicorn),
            "VALEFOR" => Ok(Server::Valefor),
            "YOJIMBO" => Ok(Server::Yojimbo),
            "ZEROMUS" => Ok(Server::Zeromus),
            
            x => Err(ServerParseError(x.into())),
        }
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let server = match *self {
            /// NA
            //  Aether
            Server::Adamantoise => "Adamantoise",
            Server::Cactuar =>     "Cactuar",
            Server::Faerie =>      "Faerie",
            Server::Gilgamesh =>   "Gilgamesh",
            Server::Jenova =>      "Jenova",
            Server::Midgardsormr =>"Midgardsormr",
            Server::Sargatanas =>  "Sargatanas",
            Server::Siren =>       "Siren",
                                    
            //  Primal              
            Server::Behemoth =>    "Behemoth",
            Server::Excalibur =>   "Excalibur",
            Server::Exodus =>      "Exodus",
            Server::Famfrit =>     "Famfrit",
            Server::Hyperion =>    "Hyperion",
            Server::Lamia =>       "Lamia",
            Server::Leviathan =>   "Leviathan",
            Server::Ultros =>      "Ultros",
                                    
            // Crystal              
            Server::Balmung =>     "Balmung",
            Server::Brynhildr =>   "Brynhildr",
            Server::Coeurl =>      "Coeurl",
            Server::Diabolos =>    "Diabolos",
            Server::Goblin =>      "Goblin",
            Server::Malboro =>     "Malboro",
            Server::Mateus =>      "Mateus",
            Server::Zalera =>      "Zalera",
                                    
            // dynamis              
            Server::Halicarnassus => "Halicarnassus", 
            Server::Maduin =>        "Maduin",
            Server::Marilith =>      "Marilith",
            Server::Seraph =>        "Seraph",
            Server::Cuchulainn =>    "Cuchulainn",
            Server::Kraken =>        "Kraken",
            Server::Rafflesia =>     "Rafflesia",
            Server::Golem =>         "Golem",
                                    
            /// EU                  
            //  Chaos               
            Server::Cerberus =>    "Cerberus",
            Server::Louisoix =>    "Louisoix",
            Server::Moogle =>      "Moogle",
            Server::Omega =>       "Omega",
            Server::Phantom =>     "Phantom",
            Server::Ragnarok =>    "Ragnarok",
            Server::Sagittarius => "Sagittarius",
            Server::Spriggan =>    "Spriggan",
                                    
            //  Light               
            Server::Alpha =>       "Alpha",
            Server::Lich =>        "Lich",
            Server::Odin =>        "Odin",
            Server::Phoenix =>     "Phoenix",
            Server::Raiden =>      "Raiden",
            Server::Shiva =>       "Shiva",
            Server::Twintania =>   "Twintania",
            Server::Zodiark =>     "Zodiark",
                                    
            //  Shadow (Temp DC for dawntrail launch)
            Server::Innocence =>   "Innocence",
            Server::Pixie =>       "Pixie",
            Server::Titania =>     "Titania",
            Server::Tycoon =>      "Tycoon",
                                    
            /// OCE                 
            //  Materia             
            Server::Bismarck =>    "Bismarck",
            Server::Ravana =>      "Ravana",
            Server::Sephirot =>    "Sephirot",
            Server::Sophia =>      "Sophia",
            Server::Zurvan =>      "Zurvan",
                                    
            /// JP                  
            //  Elemental           
            Server::Aegis =>       "Aegis",
            Server::Atomos =>      "Atomos",
            Server::Carbuncle =>   "Carbuncle",
            Server::Garuda =>      "Garuda",
            Server::Gungnir =>     "Gungnir",
            Server::Kujata =>      "Kujata",
            Server::Tonberry =>    "Tonberry",
            Server::Typhon =>      "Typhon",
                                    
            //  Gaia                
            Server::Alexander =>   "Alexander",
            Server::Bahamut =>     "Bahamut",
            Server::Durandal =>    "Durandal",
            Server::Fenrir =>      "Fenrir",
            Server::Ifrit =>       "Ifrit",
            Server::Ridill =>      "Ridill",
            Server::Tiamat =>      "Tiamat",
            Server::Ultima =>      "Ultima",
                                    
            //  Mana                
            Server::Anima =>      "Anima",
            Server::Asura =>       "Asura",
            Server::Chocobo =>     "Chocobo",
            Server::Hades =>       "Hades",
            Server::Ixion =>       "Ixion",
            Server::Masamune =>    "Masamune",
            Server::Pandaemonium =>"Pandaemonium", 
            Server::Titan =>       "Titan",
                                    
            //  Meteor              
            Server::Belias =>      "Belias",
            Server::Mandragora =>  "Mandragora",
            Server::Ramuh =>       "Ramuh",
            Server::Shinryu =>     "Shinryu",
            Server::Unicorn =>     "Unicorn",
            Server::Valefor =>     "Valefor",
            Server::Yojimbo =>     "Yojimbo",
            Server::Zeromus =>     "Zeromus",
        };

        write!(f, "{}", server)
    }
}