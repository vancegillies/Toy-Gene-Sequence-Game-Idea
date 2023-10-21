use super::{sequencer_transformer::SequencerTransformer, attributes::Attributes};

pub struct BaseTransform;

impl SequencerTransformer for BaseTransform {
    fn transfrom(&self, gene_string: &str) -> Attributes {
        let mut attrs = Attributes::empty();

        let mut chars = gene_string.chars().peekable();

        while let Some(char1) = chars.next() {
            let char2 = match chars.peek() {
                Some(&c) => c,
                _ => break, // Exit the loop if there are no more characters
            };

            match (char1, char2) {
                // LETTER A
                ('A', 'A') => attrs.agility += 4,
                ('A', 'C') => {
                    attrs.agility += 3;
                    attrs.charisma += 1;
                }
                ('A', 'G') => {
                    attrs.agility += 3;
                    attrs.strength += 1;
                }
                ('A', 'T') => {
                    attrs.agility += 3;
                    attrs.toughness += 1;
                }
                ('A', 'N') => {
                    attrs.agility += 3;
                    attrs.intelligence += 1;
                }
                ('A', 'K') => {
                    attrs.agility += 2;
                    attrs.luck += 2;
                }
                ('A', '+') => attrs.agility += 5,
                ('A', '-') => attrs.agility -= 2,
                ('+', 'A') => attrs.agility += 6,
                ('-', 'A') => attrs.agility -= 3,
                // LETTER C
                ('C', 'C') => attrs.charisma += 4,
                ('C', 'A') => {
                    attrs.charisma += 3;
                    attrs.agility += 1;
                }
                ('C', 'G') => {
                    attrs.charisma += 3;
                    attrs.strength += 1;
                }
                ('C', 'T') => {
                    attrs.charisma += 3;
                    attrs.toughness += 1;
                }
                ('C', 'N') => {
                    attrs.charisma += 3;
                    attrs.intelligence += 1;
                }
                ('C', 'K') => {
                    attrs.charisma += 2;
                    attrs.luck += 2;
                }
                ('C', '+') => attrs.charisma += 5,
                ('C', '-') => attrs.charisma -= 2,
                ('+', 'C') => attrs.charisma += 6,
                ('-', 'C') => attrs.charisma -= 3,
                // LETTER G
                ('G', 'G') => attrs.strength += 4,
                ('G', 'A') => {
                    attrs.strength += 3;
                    attrs.agility += 1;
                }
                ('G', 'C') => {
                    attrs.strength += 3;
                    attrs.charisma += 1;
                }
                ('G', 'T') => {
                    attrs.strength += 3;
                    attrs.toughness += 1;
                }
                ('G', 'N') => {
                    attrs.strength += 3;
                    attrs.intelligence += 1;
                }
                ('G', 'K') => {
                    attrs.strength += 2;
                    attrs.luck += 2;
                }
                ('G', '+') => attrs.strength += 5,
                ('G', '-') => attrs.strength -= 2,
                ('+', 'G') => attrs.strength += 6,
                ('-', 'G') => attrs.strength -= 3,
                // LETTER T
                ('T', 'T') => attrs.toughness += 4,
                ('T', 'A') => {
                    attrs.toughness += 3;
                    attrs.agility += 1;
                }
                ('T', 'C') => {
                    attrs.toughness += 3;
                    attrs.charisma += 1;
                }
                ('T', 'G') => {
                    attrs.toughness += 3;
                    attrs.strength += 1;
                }
                ('T', 'N') => {
                    attrs.toughness += 3;
                    attrs.intelligence += 1;
                }
                ('T', 'K') => {
                    attrs.toughness += 2;
                    attrs.luck += 2;
                }
                ('T', '+') => attrs.toughness += 5,
                ('T', '-') => attrs.toughness -= 2,
                ('+', 'T') => attrs.toughness += 6,
                ('-', 'T') => attrs.toughness -= 3,
                // LETTER N
                ('N', 'N') => attrs.intelligence += 4,
                ('N', 'A') => {
                    attrs.intelligence += 3;
                    attrs.agility += 1;
                }
                ('N', 'C') => {
                    attrs.intelligence += 3;
                    attrs.charisma += 1;
                }
                ('N', 'G') => {
                    attrs.intelligence += 3;
                    attrs.strength += 1;
                }
                ('N', 'T') => {
                    attrs.intelligence += 3;
                    attrs.toughness += 1;
                }
                ('N', 'K') => {
                    attrs.intelligence += 2;
                    attrs.luck += 2;
                }
                ('N', '+') => attrs.intelligence += 5,
                ('N', '-') => attrs.intelligence -= 2,
                ('+', 'N') => attrs.intelligence += 6,
                ('-', 'N') => attrs.intelligence -= 3,
                // LETTER K
                ('K', 'K') => attrs.luck += 4,
                ('K', 'A') => {
                    attrs.luck += 3;
                    attrs.agility += 1;
                }
                ('K', 'C') => {
                    attrs.luck += 3;
                    attrs.charisma += 1;
                }
                ('K', 'G') => {
                    attrs.luck += 3;
                    attrs.strength += 1;
                }
                ('K', 'T') => {
                    attrs.luck += 3;
                    attrs.toughness += 1;
                }
                ('K', 'N') => {
                    attrs.luck += 3;
                    attrs.intelligence += 1;
                }
                ('K', '+') => attrs.luck += 5,
                ('K', '-') => attrs.luck -= 2,
                ('+', 'K') => attrs.luck += 6,
                ('-', 'K') => attrs.luck -= 3,
                // LETTER +
                ('+', '+') => {
                    attrs.agility += 2;
                    attrs.charisma += 2;
                    attrs.strength += 2;
                    attrs.toughness += 2;
                    attrs.intelligence += 2;
                    attrs.luck += 2;
                }
                ('+', '-') => {
                    attrs.agility -= 1;
                    attrs.charisma -= 1;
                    attrs.strength -= 1;
                    attrs.toughness -= 1;
                    attrs.intelligence -= 1;
                    attrs.luck -= 1;
                }
                ('-', '+') => {
                    attrs.agility -= 1;
                    attrs.charisma -= 1;
                    attrs.strength -= 1;
                    attrs.toughness -= 1;
                    attrs.intelligence -= 1;
                    attrs.luck -= 1;
                }
                ('-', '-') => {
                    attrs.agility -= 2;
                    attrs.charisma -= 2;
                    attrs.strength -= 2;
                    attrs.toughness -= 2;
                    attrs.intelligence -= 2;
                    attrs.luck -= 2;
                }
                // Add more cases for other character pairs here
                _ => {
                    println!("Unknown Pair");
                    println!("{:?}", (char1, char2));
                }
            }

            // Consume the peeked character
            chars.next();
        }

        attrs
    }
}
