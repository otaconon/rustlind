use rustlind::{Codon, Strand, StrandType};

use std::{error::Error, io::BufRead};
use std::io::{Write};

use rustlind::{FileIO};

fn rna_to_protein(strand: &Strand) -> Result<Vec<Codon>, Box<dyn Error>> {
    let protein: Vec<Codon> = Vec::new();
    for idx in (0..strand.seq().len()).step_by(3) {
        let codon = Codon::from_strand(strand.seq()[idx, idx+3]).ok_or_else(|| "Inavlid strand passed to codon")?;
        protein.push();
    }
    Ok(protein)
}

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
