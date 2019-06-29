#[allow(unused)]

pub mod model;
pub mod search;

// Lazy static client to avoid creating new ones every time
lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::model::profile::Profile;

        assert!(Profile::get(14952101).is_ok());
    }

    #[test]
    fn search_works() {
        use crate::model::datacenter::Datacenter;
        use crate::search::SearchBuilder;

        let profiles = SearchBuilder::new()
            .character("Raspberry Custard")
            .datacenter(Datacenter::Primal)
            .send()
            .unwrap();

        assert_eq!(profiles.len(), 1);
    }
}