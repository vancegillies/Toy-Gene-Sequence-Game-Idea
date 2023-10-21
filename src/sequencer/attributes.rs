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

    pub fn to_binary(&self) -> String {
        format!(
            "{:08b}{:08b}{:08b}{:08b}{:08b}{:08b}",
            self.agility,
            self.charisma,
            self.strength,
            self.toughness,
            self.intelligence,
            self.luck
        )
    }

    pub fn from_binary(binary: &str) -> Attributes {
        let mut chars = binary.chars();
        let mut agility = 0;
        let mut charisma = 0;
        let mut strength = 0;
        let mut toughness = 0;
        let mut intelligence = 0;
        let mut luck = 0;

        for _ in 0..8 {
            agility = agility << 1;
            agility += match chars.next().unwrap() {
                '1' => 1,
                _ => 0,
            };
        }

        for _ in 0..8 {
            charisma = charisma << 1;
            charisma += match chars.next().unwrap() {
                '1' => 1,
                _ => 0,
            };
        }

        for _ in 0..8 {
            strength = strength << 1;
            strength += match chars.next().unwrap() {
                '1' => 1,
                _ => 0,
            };
        }

        for _ in 0..8 {
            toughness = toughness << 1;
            toughness += match chars.next().unwrap() {
                '1' => 1,
                _ => 0,
            };
        }

        for _ in 0..8 {
            intelligence = intelligence << 1;
            intelligence += match chars.next().unwrap() {
                '1' => 1,
                _ => 0,
            };
        }

        for _ in 0..8 {
            luck = luck << 1;
            luck += match chars.next().unwrap() {
                '1' => 1,
                _ => 0,
            };
        }

        Attributes {
            agility,
            charisma,
            strength,
            toughness,
            intelligence,
            luck,
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
