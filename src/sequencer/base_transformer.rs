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
                ('A', 'A') => attrs.try_add("agility", 4),
                ('A', 'C') => {
                    attrs.try_add("agility", 3);
                    attrs.try_add("charisma", 1);
                }
                ('A', 'G') => {
                    attrs.try_add("agility", 3);
                    attrs.try_add("strength", 1);
                }
                ('A', 'T') => {
                    attrs.try_add("agility", 3);
                    attrs.try_add("toughness", 1);
                }
                ('A', 'N') => {
                    attrs.try_add("agility", 3);
                    attrs.try_add("intelligence", 1);
                }
                ('A', 'K') => {
                    attrs.try_add("agility", 2);
                    attrs.try_add("luck", 2);
                }
                ('A', '+') => attrs.try_add("agility", 5),
                ('A', '-') => attrs.try_sub("agility", 2),
                ('+', 'A') => attrs.try_add("agility", 6),
                ('-', 'A') => attrs.try_sub("agility", 3),
                // LETTER C
                ('C', 'C') => attrs.try_add("charisma", 4),
                ('C', 'A') => {
                    attrs.try_add("charisma", 3);
                    attrs.try_add("agility", 1);
                }
                ('C', 'G') => {
                    attrs.try_add("charisma", 3);
                    attrs.try_add("strength", 1);
                }
                ('C', 'T') => {
                attrs.try_add("charisma", 3);
                    attrs.try_add("toughness", 1);
                }
                ('C', 'N') => {
                    attrs.try_add("charisma", 3);
                    attrs.try_add("intelligence", 1);
                }
                ('C', 'K') => {
                attrs.try_add("charisma", 2);
                    attrs.try_add("luck", 2);
                }
                ('C', '+') => attrs.try_add("charisma", 5),
                ('C', '-') => attrs.try_sub("charisma", 2),
                ('+', 'C') => attrs.try_add("charisma", 6),
                ('-', 'C') => attrs.try_sub("charisma", 3),
                // LETTER G
                ('G', 'G') => attrs.try_add("strength", 4),
                ('G', 'A') => {
                    attrs.try_add("strength", 3);
                    attrs.try_add("agility", 1);
                }
                ('G', 'C') => {
                    attrs.try_add("strength", 3);
                    attrs.try_add("charisma", 1);
                }
                ('G', 'T') => {
                    attrs.try_add("strength", 3);
                    attrs.try_add("toughness", 1);
                }
                ('G', 'N') => {
                attrs.try_add("strength", 3);
                    attrs.try_add("intelligence", 1);
                }
                ('G', 'K') => {
                    attrs.try_add("strength", 2);
                    attrs.try_add("luck", 2);
                }
                ('G', '+') => attrs.try_add("strength", 5),
                ('G', '-') => attrs.try_sub("strength", 2),
                ('+', 'G') => attrs.try_add("strength", 6),
                ('-', 'G') => attrs.try_sub("strength", 3),
                // LETTER T
                ('T', 'T') => attrs.try_add("toughness", 4),
                ('T', 'A') => {
                    attrs.try_add("toughness", 3);
                    attrs.try_add("agility", 1);
                }
                ('T', 'C') => {
                    attrs.try_add("toughness", 3);
                    attrs.try_add("charisma", 1);
                }
                ('T', 'G') => {
                    attrs.try_add("toughness", 3);
                    attrs.try_add("strength", 1);
                }
                ('T', 'N') => {
                    attrs.try_add("toughness", 3);
                    attrs.try_add("intelligence", 1);
                }
                ('T', 'K') => {
                attrs.try_add("toughness", 2);
                    attrs.try_add("luck", 2);
                }
                ('T', '+') => attrs.try_add("toughness", 5),
                ('T', '-') => attrs.try_sub("toughness", 2),
                ('+', 'T') => attrs.try_add("toughness", 6),
                ('-', 'T') => attrs.try_sub("toughness", 3),
                // LETTER N
                ('N', 'N') => attrs.try_add("intelligence", 4),
                ('N', 'A') => {
                    attrs.try_add("intelligence", 3);
                    attrs.try_add("agility", 1);
                }
                ('N', 'C') => {
                    attrs.try_add("intelligence", 3);
                    attrs.try_add("charisma", 1);
                }
                ('N', 'G') => {
                    attrs.try_add("intelligence", 3);
                    attrs.try_add("strength", 1);
                }
                ('N', 'T') => {
                    attrs.try_add("intelligence", 3);
                    attrs.try_add("toughness", 1);
                }
                ('N', 'K') => {
                    attrs.try_add("intelligence", 2);
                    attrs.try_add("luck", 2);
                }
                ('N', '+') => attrs.try_add("intelligence", 5),
                ('N', '-') => attrs.try_sub("intelligence", 2),
                ('+', 'N') => attrs.try_add("intelligence", 6),
                ('-', 'N') => attrs.try_sub("intelligence", 3),
                // LETTER K
                ('K', 'K') => attrs.try_add("luck", 4),
                ('K', 'A') => {
                    attrs.try_add("luck", 3);
                    attrs.try_add("agility", 1);
                }
                ('K', 'C') => {
                    attrs.try_add("luck", 3);
                    attrs.try_add("charisma", 1);
                }
                ('K', 'G') => {
                    attrs.try_add("luck", 3);
                    attrs.try_add("strength", 1);
                }
                ('K', 'T') => {
                    attrs.try_add("luck", 3);
                    attrs.try_add("toughness", 1);
                }
                ('K', 'N') => {
                    attrs.try_add("luck", 3);
                    attrs.try_add("intelligence", 1);
                }
                ('K', '+') => attrs.try_add("luck", 5),
                ('K', '-') => attrs.try_sub("luck", 2),
                ('+', 'K') => attrs.try_add("luck", 6),
                ('-', 'K') => attrs.try_sub("luck", 3),
                // LETTER +
                ('+', '+') => {
                    attrs.try_add("agility", 2);
                    attrs.try_add("charisma", 2);
                    attrs.try_add("strength", 2);
                    attrs.try_add("toughness", 2);
                    attrs.try_add("intelligence", 2);
                    attrs.try_add("luck", 2);
                }
                ('+', '-') => {
                    attrs.try_sub("agility", 1);
                    attrs.try_sub("charisma", 1);
                    attrs.try_sub("strength", 1);
                    attrs.try_sub("toughness", 1);
                    attrs.try_sub("intelligence", 1);
                    attrs.try_sub("luck", 1);
                }
                ('-', '+') => {
                    attrs.try_sub("agility", 1);
                    attrs.try_sub("charisma", 1);
                    attrs.try_sub("strength", 1);
                    attrs.try_sub("toughness", 1);
                    attrs.try_sub("intelligence", 1);
                    attrs.try_sub("luck", 1);
                }
                ('-', '-') => {
                    attrs.try_sub("agility", 2);
                    attrs.try_sub("charisma", 2);
                    attrs.try_sub("strength", 2);
                    attrs.try_sub("toughness", 2);
                    attrs.try_sub("intelligence", 2);
                    attrs.try_sub("luck", 2);
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
