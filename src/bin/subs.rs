use std::collections::VecDeque;
use std::io::{Write};
use std::error::Error;

use rustlind::{FileIO, Nucleobase, Scanner, Strand, StrandType};

fn find_motifs(dna: &Strand, motif: &Strand) -> Vec<usize> {
  let mut positions: Vec<usize> = Vec::new();
  let mut sub_dna: VecDeque<Nucleobase> = VecDeque::new();
  for (idx, nucleo) in dna.seq().iter().enumerate() {
    if sub_dna.len() >= motif.seq().len() {
      sub_dna.pop_front();
    }
    sub_dna.push_back(nucleo.clone());
    if sub_dna == *motif.seq() {
      positions.push(idx-sub_dna.len()+2);
    }
  }
  positions
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut io = FileIO::from_args()?;
    let mut scanner = Scanner::new(io.reader);
    let dna_str = scanner.next::<String>()?;
    let motif_str = scanner.next::<String>()?;
    let dna = Strand::from_bytes(dna_str, StrandType::DNA);
    let motif = Strand::from_bytes(motif_str, StrandType::DNA);

    writeln!(io.writer, "{}", find_motifs(&dna, &motif).iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mendels_first_law() {
      let dna = Strand::from_bytes("GATATATGCATATACTT", StrandType::DNA);
      let motif = Strand::from_bytes("ATAT", StrandType::DNA);
      assert_eq!(find_motifs(&dna, &motif), vec![2, 4, 10]);
    }
}