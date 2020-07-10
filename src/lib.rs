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

    #[test]
    fn profile_is_correct() {
        use crate::model::{
            clan::Clan,
            class::ClassType,
            datacenter::Datacenter,
            gc::GrandCompany,
            gender::Gender,
            language::Language,
            race::Race,
        };
        use crate::search::SearchBuilder;

        let profiles = SearchBuilder::new()
            .character("Strawberry Custard")
            .datacenter(Datacenter::Primal)
            .lang(Language::English)
            .grand_company(GrandCompany::Maelstrom)
            .send()
            .unwrap();

        assert_eq!(profiles.len(), 1);

        let strawberry = profiles.get(0).unwrap();

        assert_eq!(strawberry.name, "Strawberry Custard");

        assert_eq!(strawberry.race, Race::Lalafell);
        assert_eq!(strawberry.clan, Clan::Plainsfolk);
        assert_eq!(strawberry.gender, Gender::Female);
        assert_eq!(strawberry.level(ClassType::BlackMage), Some(70));

        assert_eq!(strawberry.hp, 45835);
        assert_eq!(strawberry.mp, 10000);

        let attribs = &strawberry.attributes;

        assert_eq!(attribs.get("Strength").unwrap().level, 130);
        assert_eq!(attribs.get("Dexterity").unwrap().level, 295);
        assert_eq!(attribs.get("Vitality").unwrap().level, 2937);
        assert_eq!(attribs.get("Intelligence").unwrap().level, 3031);
        assert_eq!(attribs.get("Mind").unwrap().level, 219);
        
        assert_eq!(attribs.get("Critical Hit Rate").unwrap().level, 2050);
        assert_eq!(attribs.get("Determination").unwrap().level, 1014);
        assert_eq!(attribs.get("Direct Hit Rate").unwrap().level, 1183);
        
        assert_eq!(attribs.get("Defense").unwrap().level, 2238);
        assert_eq!(attribs.get("Magic Defense").unwrap().level, 3912);
        
        assert_eq!(attribs.get("Attack Power").unwrap().level, 130);
        assert_eq!(attribs.get("Skill Speed").unwrap().level, 364);
        
        assert_eq!(attribs.get("Attack Magic Potency").unwrap().level, 3031);
        assert_eq!(attribs.get("Healing Magic Potency").unwrap().level, 219);
        assert_eq!(attribs.get("Spell Speed").unwrap().level, 1856);
        
        assert_eq!(attribs.get("Tenacity").unwrap().level, 364);
        assert_eq!(attribs.get("Piety").unwrap().level, 292);

        assert_eq!(attribs.get("Invalid Attribute"), None);
    }
}