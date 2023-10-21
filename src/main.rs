mod gene;
mod sequencer;

use sequencer::{Sequencer, attributes::Attributes};


fn main() {
    let sequencer = Sequencer::new();
    // generate 1000 random gene strings and their attributes
    for _ in 0..1000 {
        let gene_string = gene::generate_sequence();
        let attributes = sequencer.sequence(&gene_string);
        println!("{}: {}", gene::sequence_to_binary(gene_string.as_str()), attributes.to_binary());
    }
}
