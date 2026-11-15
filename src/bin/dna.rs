use std::io::{Write};
use std::error::Error;

use rustlind::{FileIO, Nucleobase, Strand, StrandType};

fn main() -> Result<(), Box<dyn Error>> {
    let mut io = FileIO::from_args()?;
    let seq = io.read()?;
    let strand = Strand::from_bytes(seq, StrandType::DNA);
    let count = strand.count();
    writeln!(io.writer, "{} {} {} {}", 
        count[Nucleobase::A],
        count[Nucleobase::C],
        count[Nucleobase::G],
        count[Nucleobase::T],
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
        let count = strand.count();
        assert_eq!(count[Nucleobase::A], 20);
        assert_eq!(count[Nucleobase::C], 12);
        assert_eq!(count[Nucleobase::G], 17);
        assert_eq!(count[Nucleobase::T], 21);
    }
}