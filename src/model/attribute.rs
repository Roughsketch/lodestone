use failure::Fail;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Attribute {
    level: u16
}

/// Holds information about a profiles level in a particular class.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attributes(HashMap<String, Attribute>);

impl Attributes {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    /// Adds or updates a given entry.
    pub fn insert(&mut self, name: String, value: Attribute) {
        self.0.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Attribute> {
        self.0.get(name)
    }
}

