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
                ('A', 'A') => attrs.agility += 4.0,
                ('A', 'C') => {
                    attrs.agility += 3.0;
                    attrs.charisma += 1.0;
                }
                ('A', 'G') => {
                    attrs.agility += 3.0;
                    attrs.strength += 1.0;
                }
                ('A', 'T') => {
                    attrs.agility += 3.0;
                    attrs.toughness += 1.0;
                }
                ('A', 'N') => {
                    attrs.agility += 3.0;
                    attrs.intelligence += 1.0;
                }
                ('A', 'K') => {
                    attrs.agility += 2.0;
                    attrs.luck += 2.0;
                }
                ('A', '+') => attrs.agility += 5.0,
                ('A', '-') => attrs.agility -= 2.0,
                ('+', 'A') => attrs.agility += 6.0,
                ('-', 'A') => attrs.agility -= 3.0,
                // LETTER C
                ('C', 'C') => attrs.charisma += 4.0,
                ('C', 'A') => {
                    attrs.charisma += 3.0;
                    attrs.agility += 1.0;
                }
                ('C', 'G') => {
                    attrs.charisma += 3.0;
                    attrs.strength += 1.0;
                }
                ('C', 'T') => {
                    attrs.charisma += 3.0;
                    attrs.toughness += 1.0;
                }
                ('C', 'N') => {
                    attrs.charisma += 3.0;
                    attrs.intelligence += 1.0;
                }
                ('C', 'K') => {
                    attrs.charisma += 2.0;
                    attrs.luck += 2.0;
                }
                ('C', '+') => attrs.charisma += 5.0,
                ('C', '-') => attrs.charisma -= 2.0,
                ('+', 'C') => attrs.charisma += 6.0,
                ('-', 'C') => attrs.charisma -= 3.0,
                // LETTER G
                ('G', 'G') => attrs.strength += 4.0,
                ('G', 'A') => {
                    attrs.strength += 3.0;
                    attrs.agility += 1.0;
                }
                ('G', 'C') => {
                    attrs.strength += 3.0;
                    attrs.charisma += 1.0;
                }
                ('G', 'T') => {
                    attrs.strength += 3.0;
                    attrs.toughness += 1.0;
                }
                ('G', 'N') => {
                    attrs.strength += 3.0;
                    attrs.intelligence += 1.0;
                }
                ('G', 'K') => {
                    attrs.strength += 2.0;
                    attrs.luck += 2.0;
                }
                ('G', '+') => attrs.strength += 5.0,
                ('G', '-') => attrs.strength -= 2.0,
                ('+', 'G') => attrs.strength += 6.0,
                ('-', 'G') => attrs.strength -= 3.0,
                // LETTER T
                ('T', 'T') => attrs.toughness += 4.0,
                ('T', 'A') => {
                    attrs.toughness += 3.0;
                    attrs.agility += 1.0;
                }
                ('T', 'C') => {
                    attrs.toughness += 3.0;
                    attrs.charisma += 1.0;
                }
                ('T', 'G') => {
                    attrs.toughness += 3.0;
                    attrs.strength += 1.0;
                }
                ('T', 'N') => {
                    attrs.toughness += 3.0;
                    attrs.intelligence += 1.0;
                }
                ('T', 'K') => {
                    attrs.toughness += 2.0;
                    attrs.luck += 2.0;
                }
                ('T', '+') => attrs.toughness += 5.0,
                ('T', '-') => attrs.toughness -= 2.0,
                ('+', 'T') => attrs.toughness += 6.0,
                ('-', 'T') => attrs.toughness -= 3.0,
                // LETTER N
                ('N', 'N') => attrs.intelligence += 4.0,
                ('N', 'A') => {
                    attrs.intelligence += 3.0;
                    attrs.agility += 1.0;
                }
                ('N', 'C') => {
                    attrs.intelligence += 3.0;
                    attrs.charisma += 1.0;
                }
                ('N', 'G') => {
                    attrs.intelligence += 3.0;
                    attrs.strength += 1.0;
                }
                ('N', 'T') => {
                    attrs.intelligence += 3.0;
                    attrs.toughness += 1.0;
                }
                ('N', 'K') => {
                    attrs.intelligence += 2.0;
                    attrs.luck += 2.0;
                }
                ('N', '+') => attrs.intelligence += 5.0,
                ('N', '-') => attrs.intelligence -= 2.0,
                ('+', 'N') => attrs.intelligence += 6.0,
                ('-', 'N') => attrs.intelligence -= 3.0,
                // LETTER K
                ('K', 'K') => attrs.luck += 4.0,
                ('K', 'A') => {
                    attrs.luck += 3.0;
                    attrs.agility += 1.0;
                }
                ('K', 'C') => {
                    attrs.luck += 3.0;
                    attrs.charisma += 1.0;
                }
                ('K', 'G') => {
                    attrs.luck += 3.0;
                    attrs.strength += 1.0;
                }
                ('K', 'T') => {
                    attrs.luck += 3.0;
                    attrs.toughness += 1.0;
                }
                ('K', 'N') => {
                    attrs.luck += 3.0;
                    attrs.intelligence += 1.0;
                }
                ('K', '+') => attrs.luck += 5.0,
                ('K', '-') => attrs.luck -= 2.0,
                ('+', 'K') => attrs.luck += 6.0,
                ('-', 'K') => attrs.luck -= 3.0,
                // LETTER +
                ('+', '+') => {
                    attrs.agility += 2.0;
                    attrs.charisma += 2.0;
                    attrs.strength += 2.0;
                    attrs.toughness += 2.0;
                    attrs.intelligence += 2.0;
                    attrs.luck += 2.0;
                }
                ('+', '-') => {
                    attrs.agility -= 1.0;
                    attrs.charisma -= 1.0;
                    attrs.strength -= 1.0;
                    attrs.toughness -= 1.0;
                    attrs.intelligence -= 1.0;
                    attrs.luck -= 1.0;
                }
                ('-', '+') => {
                    attrs.agility -= 1.0;
                    attrs.charisma -= 1.0;
                    attrs.strength -= 1.0;
                    attrs.toughness -= 1.0;
                    attrs.intelligence -= 1.0;
                    attrs.luck -= 1.0;
                }
                ('-', '-') => {
                    attrs.agility -= 2.0;
                    attrs.charisma -= 2.0;
                    attrs.strength -= 2.0;
                    attrs.toughness -= 2.0;
                    attrs.intelligence -= 2.0;
                    attrs.luck -= 2.0;
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
