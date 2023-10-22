use std::fs::File;
use std::io::{Write, Result};

pub fn write_to_csv(filename: &str, data: Vec<(String, String)>) -> Result<()> {
    let mut file = File::create(filename)?;

    // Write the header row
    writeln!(file, "Input,Output")?;

    // Write the data
    for record in data {
        writeln!(file, "{},{}", record.0, record.1)?;
    }

    Ok(())
}
