mod gene;
mod sequencer;

use sequencer::Sequencer;


fn main() {
    // let sequencer = Sequencer::new();
    // let seq = gene::generate_sequence();
    // let bin = gene::sequence_to_binary(&seq);
    println!("{}", gene::sequence_from_binary("110010100010001001101100100100001010010100101001001101001101100001001001100100110100010010010011001010011101100100011001"));

}
