mod gene;
mod sequencer;

use sequencer::{Sequencer, attributes::Attributes};


fn main() {
    let sequencer = Sequencer::new();
    let seq = gene::generate_sequence();
    let attrs = sequencer.sequence(&seq);
    println!("{:?} {:?}", attrs, Attributes::from_binary(attrs.to_binary().as_str()));

}
