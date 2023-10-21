mod gene;
mod sequencer;

use sequencer::Sequencer;


fn main() {
    // let sequencer = Sequencer::new();
    let seq = gene::generate_sequence();
    let bin = gene::sequence_to_binary(&seq);
    println!("{} {}", seq, bin);

}
