#[allow(unused)]

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
        use model::profile::Profile;

        assert!(Profile::get(14952101).is_ok());
    }

    #[test]
    fn search_works() {
        use model::server::Server;
        use search::SearchBuilder;

        let profiles = SearchBuilder::new()
            .character("Raspberry Custard")
            .server(Server::Famfrit)
            .send()
            .unwrap();

        assert_eq!(profiles.len(), 1);
    }
}