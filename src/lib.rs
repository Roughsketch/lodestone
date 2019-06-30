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
    fn can_grab_profile() {
        use crate::model::profile::Profile;

        assert!(Profile::get(11908971).is_ok());
    }

    #[test]
    fn can_create_search() {
        use crate::model::datacenter::Datacenter;
        use crate::model::gc::GrandCompany;
        use crate::model::language::Language;
        use crate::search::SearchBuilder;

        let profiles = SearchBuilder::new()
            .character("Strawberry Custard")
            .datacenter(Datacenter::Primal)
            .lang(Language::English)
            .grand_company(GrandCompany::Maelstrom)
            .send()
            .unwrap();

        assert_eq!(profiles.len(), 1);
    }
}