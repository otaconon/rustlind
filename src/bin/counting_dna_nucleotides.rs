use std::io::{Write};
use std::error::Error;

use rustlind::FileIO;

fn count_nucleobases(buf: &[u8]) -> [usize; 256] {
    let mut counts = [0usize; 256];
    for &byte in buf {
        counts[byte as usize] += 1
    }
    counts
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut io = FileIO::from_args()?;
    let buf = io.read()?;
    let counts = count_nucleobases(&buf);
    writeln!(io.writer, "{} {} {} {}", counts[b'A' as usize], counts[b'C' as usize], counts[b'G' as usize], counts[b'T' as usize])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_nucleobases() {
        let input = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
        let result = count_nucleobases(input.as_bytes());
        assert_eq!(result[b'A' as usize], 20);
        assert_eq!(result[b'C' as usize], 12);
        assert_eq!(result[b'G' as usize], 17);
        assert_eq!(result[b'T' as usize], 21);
    }
}