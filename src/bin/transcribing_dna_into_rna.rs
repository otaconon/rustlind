use std::io::{Write};
use std::error::Error;

use rustlind::FileIO;

fn transcribe_dna_to_rna(dna: &[u8]) -> String {
    let mut rna = String::new();
    for &byte in dna {
        match byte {
            b'T' => rna.push('U'),
            _ => rna.push(byte as char),
        }
    }
    rna
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut io = FileIO::from_args()?;
    let dna = io.read()?;
    let rna = transcribe_dna_to_rna(&dna);
    writeln!(io.writer, "{}", rna)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcribe_dna_to_rna() {
        let input = "GATGGAACTTGACTACGTAAATT";
        let result = transcribe_dna_to_rna(input.as_bytes());
        assert_eq!(result, "GAUGGAACUUGACUACGUAAAUU")
    }
}