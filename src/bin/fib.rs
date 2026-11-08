use std::io::{Write};
use std::error::Error;

use rustlind::{FileIO, Scanner};

struct Generation {
    child_pairs: u32,
    adult_pairs: u32
}

fn next_gen(mut g: Generation, child_mult: u32, steps: u32) -> Generation {
    if steps <= 1 {
        return g
    }
    let new_adult_pairs = g.child_pairs;
    g.child_pairs = g.adult_pairs*child_mult;
    g.adult_pairs += new_adult_pairs;
    next_gen(g, child_mult, steps-1)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut io = FileIO::from_args()?;
    let mut scanner = Scanner::new(io.reader);
    let n = scanner.next::<u32>()?;
    let k = scanner.next::<u32>()?;
    let first_gen = Generation{child_pairs: 1, adult_pairs: 0};
    let last_gen = next_gen(first_gen, k, n);
    writeln!(io.writer, "{}", last_gen.adult_pairs+last_gen.child_pairs)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcribe_dna_to_rna() {
        let [n, k]: [u32; 2] = [5, 3];
        let first_gen = Generation{child_pairs: 1, adult_pairs: 0};
        let last_gen = next_gen(first_gen, k, n);
        assert_eq!(last_gen.adult_pairs+last_gen.child_pairs, 19);
    }
}