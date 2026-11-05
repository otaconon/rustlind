use std::io::{Write};
use std::error::Error;

use rustlind::{FileIO, Nucleobase, Strand, StrandType};

fn main() -> Result<(), Box<dyn Error>> {
    let mut io = FileIO::from_args()?;
    let seq = io.read()?;
    let strand = Strand::from_bytes(seq, StrandType::DNA);
    writeln!(io.writer, "{} {} {} {}", 
        strand.nucleotide_infos()[Nucleobase::A].positions.len(),
        strand.nucleotide_infos()[Nucleobase::C].positions.len(),
        strand.nucleotide_infos()[Nucleobase::G].positions.len(),
        strand.nucleotide_infos()[Nucleobase::T].positions.len()
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_nucleobases() {
        let input = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
        let strand = Strand::from_bytes(input, StrandType::DNA);
        assert_eq!(strand.nucleotide_infos()[Nucleobase::A].positions.len(), 20);
        assert_eq!(strand.nucleotide_infos()[Nucleobase::C].positions.len(), 12);
        assert_eq!(strand.nucleotide_infos()[Nucleobase::G].positions.len(), 17);
        assert_eq!(strand.nucleotide_infos()[Nucleobase::T].positions.len(), 21);
    }
}