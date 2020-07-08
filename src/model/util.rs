use failure::Error;
use select::document::Document;

use crate::CLIENT;

/// The URL base for profiles.
static BASE_PROFILE_URL: &str = "https://na.finalfantasyxiv.com/lodestone/character/";

pub(crate) fn load_url(user_id: u32, subpage: Option<&str>) -> Result<Document, Error> {
    let subpage = match subpage {
        None => "".to_string(),
        Some(v) => format!("{}/", v)
    };
    let mut response = CLIENT.get(&format!("{}{}/{}", BASE_PROFILE_URL, user_id, subpage)).send()?;
    let text = response.text()?;
    Ok(Document::from(text.as_str()))
}

