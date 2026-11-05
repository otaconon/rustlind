use std::io::{Write};
use std::error::Error;

use rustlind::{FileIO, Strand, StrandType};

fn main() -> Result<(), Box<dyn Error>> {
    let mut io = FileIO::from_args()?;
    let dna = io.read()?;
    let rna = Strand::from_bytes(dna, StrandType::DNA).to_rna();
    writeln!(io.writer, "{}", rna)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcribe_dna_to_rna() -> Result<(), Box<dyn Error>>{
        let dna = "GATGGAACTTGACTACGTAAATT";
        let rna = Strand::from_bytes(dna, StrandType::DNA).to_rna();
        assert_eq!(rna.to_string(), "GAUGGAACUUGACUACGUAAAUU");
        Ok(())
    }
}