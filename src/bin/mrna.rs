use std::error::Error;
use std::io::Write;

use rustlind::{AminoAcid, Codon, FileIO, Protein, Scanner};

fn count_rna_combs(prot: &Protein) -> u128 {
    let mut combs: u128 = 1;
    for amino_acid in prot.seq() {
        combs *= match amino_acid {
            AminoAcid::Met | AminoAcid::Trp => 1,
            AminoAcid::Asn
            | AminoAcid::Asp
            | AminoAcid::Cys
            | AminoAcid::Gln
            | AminoAcid::Glu
            | AminoAcid::His
            | AminoAcid::Lys
            | AminoAcid::Phe
            | AminoAcid::Tyr => 2,
            AminoAcid::Ile | AminoAcid::Stop => 3,
            AminoAcid::Ala | AminoAcid::Gly | AminoAcid::Pro | AminoAcid::Thr | AminoAcid::Val => 4,
            AminoAcid::Arg | AminoAcid::Leu | AminoAcid::Ser => 6,
        };
        combs %= 1_000_000;
    }
    return combs;
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut io = FileIO::from_args()?;
    let mut seq = io.read()?;
    seq.push(b'*');
    let prot = Protein::from_bytes(seq);
    writeln!(io.writer, "{}", count_rna_combs(&prot))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_rna_combs() {
        let prot = Protein::from_bytes("MA*");
        assert_eq!(count_rna_combs(&prot), 12);
    }
}
