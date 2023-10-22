mod gene;
mod sequencer;
mod csv;

use sequencer::{Sequencer, attributes::Attributes};


fn main() {
    let sequencer = Sequencer::new();
    let mut data = vec![];
    for _ in 0..10000 {
        let gene_string = gene::generate_sequence();
        let attributes = sequencer.sequence(&gene_string);
        data.push((gene::sequence_to_binary(gene_string.as_str()), attributes.to_binary()));
    }

    csv::write_to_csv("train.csv", data).unwrap();
}
