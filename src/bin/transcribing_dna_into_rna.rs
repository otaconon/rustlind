use std::io::{Write};
use std::error::Error;

use rustlind::{FileIO, Strand, StrandType};

fn main() -> Result<(), Box<dyn Error>> {
    let mut io = FileIO::from_args()?;
    let dna = io.read()?;
    let rna = Strand::new(dna, Some(StrandType::DNA))?.to_rna()?;
    writeln!(io.writer, "{:?}", rna.seq)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcribe_dna_to_rna() -> Result<(), Box<dyn Error>>{
        let dna = "GATGGAACTTGACTACGTAAATT";
        let rna = Strand::new(dna, Some(StrandType::DNA))?.to_rna()?;
        assert_eq!(String::from_utf8(rna.seq).unwrap(), "GAUGGAACUUGACUACGUAAAUU");
        Ok(())
    }
}