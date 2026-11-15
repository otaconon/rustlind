use rustlind::{fasta};

use std::{error::Error, io::BufRead};
use std::io::{Write};

use rustlind::{FileIO};

fn highest_gc<R: BufRead>(reader: R) -> Result<(String, f64), Box<dyn Error>> {
    let mut reader = fasta::Reader::new(reader);
    let mut best: Option<(String, f64)> = None;
    while let Some(rec) = reader.read_record()? {
        let gc = rec.strand.gc_content();
        if best.as_ref().map_or(true, |(_, b)| gc > *b) {
            best = Some((rec.id, gc));
        }
    }
    best.ok_or_else(|| "no records".into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut io = FileIO::from_args()?;
    let (id, gc) = highest_gc(io.reader)?;
    writeln!(io.writer, "{}", id)?;
    writeln!(io.writer, "{:.5}", gc*100.0)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highest_gc() {
        let input = ">Rosalind_6404
CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
TCCCACTAATAATTCTGAGG
>Rosalind_5959
CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
ATATCCATTTGTCAGCAGACACGC
>Rosalind_0808
CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
TGGGAACCTGCGGGCAGTAGGTGGAAT";
        let (id, gc) = highest_gc(input.as_bytes()).unwrap();
        assert_eq!(id, "Rosalind_0808");
        rustlind::assert_delta!(gc*100.0, 60.919540, 1e-3);
    }
}
