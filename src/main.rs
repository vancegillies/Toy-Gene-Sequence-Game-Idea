mod gene;
mod sequencer;

use sequencer::Sequencer;


fn main() {
    let sequencer = Sequencer::new();
    for _ in 0..100000 {
        let seq = gene::generate_sequence();
        let attrs = sequencer.sequence(&seq);
        println!("{} {:?}", seq, attrs);
    }

}
