use std::io::{Write};
use std::error::Error;

use rustlind::{FileIO, Scanner, Strand, StrandType};

fn hamming_dist(dna1: &Strand, dna2: &Strand) -> usize {
    let mut hamm = 0;
    for (nucleo1, nucleo2) in dna1.seq().iter().zip(dna2.seq()) {
        if nucleo1 != nucleo2 {
            hamm += 1;
        }
    }
    hamm
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut io = FileIO::from_args()?;
    let mut scanner = Scanner::new(io.reader);
    let dna1_str = scanner.next::<String>()?;
    let dna2_str = scanner.next::<String>()?;
    let dna1 = Strand::from_bytes(dna1_str, StrandType::DNA);
    let dna2 = Strand::from_bytes(dna2_str, StrandType::DNA);

    writeln!(io.writer, "{}", hamming_dist(&dna1, &dna2))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_dist() {
      let dna1 = Strand::from_bytes("GAGCCTACTAACGGGAT", StrandType::DNA);
      let dna2 = Strand::from_bytes("CATCGTAATGACGGCCT", StrandType::DNA);
      assert_eq!(hamming_dist(&dna1, &dna2), 7);
    }
}