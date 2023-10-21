use super::attributes::Attributes;

pub trait SequencerTransformer {
    fn transfrom(&self, gene_string: &str) -> Attributes;
}
