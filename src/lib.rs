#[allow(unused)]

// #[macro_use] extern crate bitflags;
#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;
extern crate reqwest;
extern crate select;

pub mod model;
pub mod search;

/// Lazy static client to avoid creating new ones every time
lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use model::{race::Race, profile::Profile, clan::Clan, gender::Gender, server::Server};

        let profile = Profile::get(14952101).unwrap();
        assert_eq!(profile.free_company, Some("Sky Pirate".to_string()));
        assert_eq!(profile.name, "Api Idyoum");
        assert_eq!(profile.server, Server::Leviathan);
        assert_eq!(profile.race, Race::Lalafell);
        assert_eq!(profile.clan, Clan::Dunesfolk);
        assert_eq!(profile.gender, Gender::Female);

        println!("{:#?}", profile);
    }

    #[test]
    fn search_works() {
        use model::{race::Race, clan::Clan, gender::Gender, server::Server};
        use search::SearchBuilder;

        let profiles = SearchBuilder::new()
            .character("Raspberry Custard")
            .server(Server::Famfrit)
            .send()
            .unwrap();

        assert_eq!(profiles.len(), 1);

        let profile = &profiles[0];

        assert_eq!(profile.free_company, Some("Goddess of Magic".to_string()));
        assert_eq!(profile.name, "Raspberry Custard");
        assert_eq!(profile.server, Server::Famfrit);
        assert_eq!(profile.race, Race::Aura);
        assert_eq!(profile.clan, Clan::Xaela);
        assert_eq!(profile.gender, Gender::Female);

        println!("{:#?}", profile);
    }
}