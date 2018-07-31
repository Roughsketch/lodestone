use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Fail)]
#[fail(display = "Invalid server string '{}'", _0)]
pub struct ServerParseError(String);

/// An enumeration for the servers that are currently available.
/// This list is taken from https://na.finalfantasyxiv.com/lodestone/worldstatus/
/// and the order should be identical.
#[derive(Clone, Debug, PartialEq)]
pub enum Server {
    //  Elemental
    Aegis,
    Atomos,
    Carbuncle,
    Garuda,
    Gungnir,
    Kujata,
    Ramuh,
    Tonberry,
    Typhon,
    Unicorn,
    //  Gaia
    Alexander,
    Bahamut,
    Durandal,
    Fenrir,
    Ifrit,
    Ridill,
    Taimat,
    Ultima,
    Valefor,
    Yojimbo,
    Zeromus,
    //  Mana
    Aniuma,
    Asura,
    Belias,
    Chocobo,
    Hades,
    Ixion,
    Mandragora,
    Masamune,
    Pandaemonium,
    Shinryu,
    Titan,
    //  Aether
    Adamantoise,
    Balmung,
    Cactuar,
    Coeurl,
    Faerie,
    Gilgamesh,
    Goblin,
    Jenova,
    Mateus,
    Midgardsormr,
    Sargatanas,
    Siren,
    Zalera,
    //  Primal
    Behemoth,
    Brynhildr,
    Diabolos,
    Excalibur,
    Exodus,
    Famfrit,
    Hyperion,
    Lamia,
    Leviathan,
    Malboro,
    Ultros,
    //  Chaos
    Cerberus,
    Lich,
    Louisoix,
    Moogle,
    Odin,
    Omega,
    Phoenix,
    Ragnarok,
    Shiva,
    Zodiark,
}

/// Case insensitive FromStr impl for servers.
impl FromStr for Server {
    type Err = ServerParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_uppercase() {
            //  Elemental
            "AEGIS" => Ok(Server::Aegis),
            "ATOMOS" => Ok(Server::Atomos),
            "CARBUNCLE" => Ok(Server::Carbuncle),
            "GARUDA" => Ok(Server::Garuda),
            "GUNGNIR" => Ok(Server::Gungnir),
            "KUJATA" => Ok(Server::Kujata),
            "RAMUH" => Ok(Server::Ramuh),
            "TONBERRY" => Ok(Server::Tonberry),
            "TYPHON" => Ok(Server::Typhon),
            "UNICORN" => Ok(Server::Unicorn),
            //  Gaia
            "ALEXANDER" => Ok(Server::Alexander),
            "BAHAMUT" => Ok(Server::Bahamut),
            "DURANDAL" => Ok(Server::Durandal),
            "FENRIR" => Ok(Server::Fenrir),
            "IFRIT" => Ok(Server::Ifrit),
            "RIDILL" => Ok(Server::Ridill),
            "TAIMAT" => Ok(Server::Taimat),
            "ULTIMA" => Ok(Server::Ultima),
            "VALEFOR" => Ok(Server::Valefor),
            "YOJIMBO" => Ok(Server::Yojimbo),
            "ZEROMUS" => Ok(Server::Zeromus),
            //  Mana
            "ANIUMA" => Ok(Server::Aniuma),
            "ASURA" => Ok(Server::Asura),
            "BELIAS" => Ok(Server::Belias),
            "CHOCOBO" => Ok(Server::Chocobo),
            "HADES" => Ok(Server::Hades),
            "IXION" => Ok(Server::Ixion),
            "MANDRAGORA" => Ok(Server::Mandragora),
            "MASAMUNE" => Ok(Server::Masamune),
            "PANDAEMONIUM" => Ok(Server::Pandaemonium),
            "SHINRYU" => Ok(Server::Shinryu),
            "TITAN" => Ok(Server::Titan),
            //  Aether
            "ADAMANTOISE" => Ok(Server::Adamantoise),
            "BALMUNG" => Ok(Server::Balmung),
            "CACTUAR" => Ok(Server::Cactuar),
            "COEURL" => Ok(Server::Coeurl),
            "FAERIE" => Ok(Server::Faerie),
            "GILGAMESH" => Ok(Server::Gilgamesh),
            "GOBLIN" => Ok(Server::Goblin),
            "JENOVA" => Ok(Server::Jenova),
            "MATEUS" => Ok(Server::Mateus),
            "MIDGARDSORMR" => Ok(Server::Midgardsormr),
            "SARGATANAS" => Ok(Server::Sargatanas),
            "SIREN" => Ok(Server::Siren),
            "ZALERA" => Ok(Server::Zalera),
            //  Primal
            "BEHEMOTH" => Ok(Server::Behemoth),
            "BRYNHILDR" => Ok(Server::Brynhildr),
            "DIABOLOS" => Ok(Server::Diabolos),
            "EXCALIBUR" => Ok(Server::Excalibur),
            "EXODUS" => Ok(Server::Exodus),
            "FAMFRIT" => Ok(Server::Famfrit),
            "HYPERION" => Ok(Server::Hyperion),
            "LAMIA" => Ok(Server::Lamia),
            "LEVIATHAN" => Ok(Server::Leviathan),
            "MALBORO" => Ok(Server::Malboro),
            "ULTROS" => Ok(Server::Ultros),
            //  Chaos
            "CERBERUS" => Ok(Server::Cerberus),
            "LICH" => Ok(Server::Lich),
            "LOUISOIX" => Ok(Server::Louisoix),
            "MOOGLE" => Ok(Server::Moogle),
            "ODIN" => Ok(Server::Odin),
            "OMEGA" => Ok(Server::Omega),
            "PHOENIX" => Ok(Server::Phoenix),
            "RAGNAROK" => Ok(Server::Ragnarok),
            "SHIVA" => Ok(Server::Shiva),
            "ZODIARK" => Ok(Server::Zodiark),

            x => Err(ServerParseError(x.into())),
        }
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let server = match *self {
            //  Elemental
            Server::Aegis => "Aegis",
            Server::Atomos => "Atomos",
            Server::Carbuncle => "Carbuncle",
            Server::Garuda => "Garuda",
            Server::Gungnir => "Gungnir",
            Server::Kujata => "Kujata",
            Server::Ramuh => "Ramuh",
            Server::Tonberry => "Tonberry",
            Server::Typhon => "Typhon",
            Server::Unicorn => "Unicorn",
            //  Gaia
            Server::Alexander => "Alexander",
            Server::Bahamut => "Bahamut",
            Server::Durandal => "Durandal",
            Server::Fenrir => "Fenrir",
            Server::Ifrit => "Ifrit",
            Server::Ridill => "Ridill",
            Server::Taimat => "Taimat",
            Server::Ultima => "Ultima",
            Server::Valefor => "Valefor",
            Server::Yojimbo => "Yojimbo",
            Server::Zeromus => "Zeromus",
            //  Mana
            Server::Aniuma => "Aniuma",
            Server::Asura => "Asura",
            Server::Belias => "Belias",
            Server::Chocobo => "Chocobo",
            Server::Hades => "Hades",
            Server::Ixion => "Ixion",
            Server::Mandragora => "Mandragora",
            Server::Masamune => "Masamune",
            Server::Pandaemonium => "Pandaemonium",
            Server::Shinryu => "Shinryu",
            Server::Titan => "Titan",
            //  Aether
            Server::Adamantoise => "Adamantoise",
            Server::Balmung => "Balmung",
            Server::Cactuar => "Cactuar",
            Server::Coeurl => "Coeurl",
            Server::Faerie => "Faerie",
            Server::Gilgamesh => "Gilgamesh",
            Server::Goblin => "Goblin",
            Server::Jenova => "Jenova",
            Server::Mateus => "Mateus",
            Server::Midgardsormr => "Midgardsormr",
            Server::Sargatanas => "Sargatanas",
            Server::Siren => "Siren",
            Server::Zalera => "Zalera",
            //  Primal
            Server::Behemoth => "Behemoth",
            Server::Brynhildr => "Brynhildr",
            Server::Diabolos => "Diabolos",
            Server::Excalibur => "Excalibur",
            Server::Exodus => "Exodus",
            Server::Famfrit => "Famfrit",
            Server::Hyperion => "Hyperion",
            Server::Lamia => "Lamia",
            Server::Leviathan => "Leviathan",
            Server::Malboro => "Malboro",
            Server::Ultros => "Ultros",
            //  Chaos
            Server::Cerberus => "Cerberus",
            Server::Lich => "Lich",
            Server::Louisoix => "Louisoix",
            Server::Moogle => "Moogle",
            Server::Odin => "Odin",
            Server::Omega => "Omega",
            Server::Phoenix => "Phoenix",
            Server::Ragnarok => "Ragnarok",
            Server::Shiva => "Shiva",
            Server::Zodiark => "Zodiark",
        };

        write!(f, "{}", server)
    }
}