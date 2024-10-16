use std::collections::HashMap;

///
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Slot {
    pub name: Option<String>,
    pub glamour_name: Option<String>,
    pub ilvl: Option<u32>,
}


#[derive(Clone, Debug, Default, Eq, PartialEq)]
//pub struct EquippedGear(HashMap<String, Slot>);
pub struct EquippedGear {
    pub mainhand:     Option<Slot>,
    pub head:         Option<Slot>,
    pub body:         Option<Slot>,
    pub hands:        Option<Slot>,
    pub legs:         Option<Slot>,
    pub feet:         Option<Slot>,
    pub facewear:     Option<Slot>,

    pub offhand:      Option<Slot>,
    pub earrings:     Option<Slot>,
    pub necklace:     Option<Slot>,
    pub bracelets:    Option<Slot>,
    pub ring_left:    Option<Slot>,
    pub ring_right:   Option<Slot>,
    pub soul_crystal: Option<Slot>,
}
