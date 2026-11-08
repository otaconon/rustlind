use std::io::{Write};
use std::error::Error;

use rustlind::{FileIO, Strand, StrandType};

fn main() -> Result<(), Box<dyn Error>> {
    let mut io = FileIO::from_args()?;
    let dna_bytes = io.read()?;
    let dna = Strand::from_bytes(dna_bytes, StrandType::DNA);
    writeln!(io.writer, "{}", dna.to_complement().to_string().chars().rev().collect::<String>())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcribe_dna_to_rna() {
        let dna_bytes = "AAAACCCGGT";
        let dna = Strand::from_bytes(dna_bytes, StrandType::DNA);
        assert_eq!(dna.to_complement().to_string().chars().rev().collect::<String>(), "ACCGGGTTTT");
    }
}