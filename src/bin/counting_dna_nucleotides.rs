use std::io::{Write};
use std::error::Error;

use rustlind::{FileIO, Nucleobase, Strand};

fn main() -> Result<(), Box<dyn Error>> {
    let mut io = FileIO::from_args()?;
    let seq = io.read()?;
    let strand = Strand::new(seq, None)?;
    writeln!(io.writer, "{} {} {} {}", 
        strand.nucleotide_infos[Nucleobase::A as usize].count,
        strand.nucleotide_infos[Nucleobase::C as usize].count,
        strand.nucleotide_infos[Nucleobase::G as usize].count,
        strand.nucleotide_infos[Nucleobase::T as usize].count
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_nucleobases() -> Result<(), Box<dyn Error>> {
        let input = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
        let strand = Strand::new(input, None)?;
        assert_eq!(strand.nucleotide_infos[Nucleobase::A as usize].count, 20);
        assert_eq!(strand.nucleotide_infos[Nucleobase::C as usize].count, 12);
        assert_eq!(strand.nucleotide_infos[Nucleobase::G as usize].count, 17);
        assert_eq!(strand.nucleotide_infos[Nucleobase::T as usize].count, 21);
        Ok(())
    }
}