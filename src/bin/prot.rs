use rustlind::{Codon, Strand, StrandType};

use std::error::Error;
use std::io::Write;

use rustlind::FileIO;

fn rna_to_protein(strand: &Strand) -> Result<Vec<Codon>, Box<dyn Error>> {
    let mut protein: Vec<Codon> = Vec::new();
    for idx in (0..strand.seq().len()-3).step_by(3) {
        let codon_strand =
            Strand::from_sequence(strand.seq()[idx..idx + 3].to_vec(), StrandType::RNA);
        let codon =
            Codon::from_strand(codon_strand.clone()).ok_or_else(|| format!("Inavlid strand passed to codon: {}", codon_strand))?;
        protein.push(codon);
    }
    Ok(protein)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut io = FileIO::from_args()?;
    let rna_bytes = io.read()?;
    let rna = Strand::from_bytes(rna_bytes, StrandType::RNA);
    let protein = rna_to_protein(&rna)?;

    writeln!(
        io.writer,
        "{}",
        protein.iter().map(|x| x.letter()).collect::<String>()
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rna_to_protein() {
        let rna_bytes = "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA";
        let rna = Strand::from_bytes(rna_bytes, StrandType::RNA);
        let protein = rna_to_protein(&rna).unwrap();
        assert_eq!(
            protein.iter().map(|x| x.letter()).collect::<String>(),
            "MAMAPRTEINSTRING"
        );
    }
}
