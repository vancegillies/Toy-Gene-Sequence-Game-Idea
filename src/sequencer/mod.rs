mod attributes;
mod sequencer_transformer;
mod base_transformer;

use self::{
    sequencer_transformer::SequencerTransformer,
    base_transformer::BaseTransform,
    attributes::Attributes
};


pub struct Sequencer {
    transformers: Vec<Box<dyn SequencerTransformer>>,
}

impl Sequencer {
    pub fn new() -> Sequencer {
        Sequencer {
            transformers: vec![
                Box::new(BaseTransform),
            ],
        }
    }

    pub fn sequence(&self, gene_string: &str) -> Attributes {
        let mut attr_layers = vec![];
        for transformer in &self.transformers {
           attr_layers.push(transformer.transfrom(gene_string));
        }
        return sum_attributes(attr_layers);
    }

}

fn sum_attributes(attrs: Vec<Attributes>) -> Attributes {
    let mut attributes = Attributes::empty();

    for attr_layer in attrs {
        attributes = attributes + attr_layer.clone();
    }

    attributes
}
