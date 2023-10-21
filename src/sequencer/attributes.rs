use std::ops::Add;

/**
* u8 should be enough for each attribute, so we can use a single byte for each attribute.
* and the output of a nn can stay an manageable size.
*/

#[derive(Debug, PartialEq, Clone)]
pub struct Attributes {
    pub agility: u8,
    pub charisma: u8,
    pub strength: u8,
    pub toughness: u8,
    pub intelligence: u8,
    pub luck: u8,
}

impl Attributes {
    pub fn empty() -> Attributes {
        Attributes {
            agility: 0,
            charisma: 0,
            strength: 0,
            toughness: 0,
            intelligence: 0,
            luck: 0,
        }
    }
}

impl Add for Attributes {
    type Output = Attributes;

    fn add(self, other: Attributes) -> Attributes {
        Attributes {
            agility: self.agility + other.agility,
            charisma: self.charisma + other.charisma,
            strength: self.strength + other.strength,
            toughness: self.toughness + other.toughness,
            intelligence: self.intelligence + other.intelligence,
            luck: self.luck + other.luck,
        }
    }
}
