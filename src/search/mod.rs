use failure::Error;
use select::document::Document;
use select::predicate::Class;

use CLIENT;
use model::profile::Profile;
use model::server::Server;

use std::fmt::Write;

static BASE_SEARCH_URL: &str = "https://na.finalfantasyxiv.com/lodestone/character/?";

pub struct SearchBuilder(String);

impl SearchBuilder {
    pub fn new() -> Self {
        SearchBuilder(BASE_SEARCH_URL.into())
    }

    pub fn send(self) -> Result<Vec<Profile>, Error> {
        let url = self.0.trim_right_matches('&');

        println!("URL: {}", url);

        let mut response = CLIENT.get(url).send()?;
        let text = response.text()?;
        let doc = Document::from(text.as_str());

        Ok(doc.find(Class("entry__link")).filter_map(|node| {
            println!("Found node");
            node.attr("href")
                .and_then(|text| {
                    println!("Got attr {}", text);
                    let digits = text.chars()
                        .skip_while(|ch| !ch.is_digit(10))
                        .take_while(|ch| ch.is_digit(10))
                        .collect::<String>();
                    
                    digits.parse::<u32>().ok()
                })
                .and_then(|id| {
                    println!("Found id {}", id);
                    let profile = Profile::get(id);

                    println!("Profile: {:?}", profile);

                    profile.ok()
                })
        }).collect())
    }

    pub fn character(mut self, name: &str) -> Self {
        let _ = write!(self.0, "q={}&", name.replace(" ", "+"));

        self
    }

    pub fn server(mut self, server: Server) -> Self {
        let _ = write!(self.0, "worldname={}&", server);

        self
    }
}