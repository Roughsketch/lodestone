use std::collections::HashMap;

/// Contains all data about an attribute; currently, this only consists of the attribute's level
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Attribute {
    /// Level of a given attribute
    pub level: u16
}

/// Holds information about a profiles level in a particular class.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Attributes(HashMap<String, Attribute>);

impl Attributes {
    pub fn new() -> Self {
        Self(HashMap::with_capacity(18))
    }
    /// Adds or updates a given entry.
    pub fn insert(&mut self, name: String, value: Attribute) {
        self.0.insert(name, value);
    }

    /// Borrows an attribute by name, if found
    pub fn get(&self, name: &str) -> Option<&Attribute> {
        self.0.get(name)
    }
}

