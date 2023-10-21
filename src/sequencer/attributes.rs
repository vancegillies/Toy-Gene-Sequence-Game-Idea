use std::ops::Add;

#[derive(Debug, PartialEq, Clone)]
pub struct Attributes {
    pub agility: f32,
    pub charisma: f32,
    pub strength: f32,
    pub toughness: f32,
    pub intelligence: f32,
    pub luck: f32,
}

impl Attributes {
    pub fn empty() -> Attributes {
        Attributes {
            agility: 0.0,
            charisma: 0.0,
            strength: 0.0,
            toughness: 0.0,
            intelligence: 0.0,
            luck: 0.0,
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
