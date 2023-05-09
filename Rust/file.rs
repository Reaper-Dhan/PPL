use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "summa.txt";
    let mut file = File::create(filename)?;
    writeln!(file, "Summa oru documenting")?;
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}

