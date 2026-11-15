use rustlind::{Nucleobase, fasta};

use std::io::{Write};
use std::error::Error;

use rustlind::{FileIO, Scanner};

fn main() -> Result<(), Box<dyn Error>> {
    let mut io = FileIO::from_args()?;
    let reader = fasta::Reader::new(io.reader);
    let best_record = reader.read_record()?.ok_or("ERROR: No records")?;
    while let Some(record) = reader.read_record()? {
        if best_record.strand.nucleotide_infos()[Nucleobase::G] + 
            best_record.strand.nucleotide_infos()[Nucleobase::C] < 
            record.strand.nucleotide_infos()[Nucleobase::G] + 
            record.strand.nucleotide_infos()[Nucleobase::C] {
                best_record = record;
            }
    }
    writeln!(io.writer, "{}", best_record.id)?;
    writeln!(io.writer, "{:.5}", best_record.strand)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) { panic!(); }
        }
    }

    #[test]
    fn test_mendels_first_law() {
        let input = ">Rosalind_6404
CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
TCCCACTAATAATTCTGAGG
>Rosalind_5959
CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
ATATCCATTTGTCAGCAGACACGC
>Rosalind_0808
CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
TGGGAACCTGCGGGCAGTAGGTGGAAT";
        assert_delta!(dominant_allele_prob(k, m, n), 0.78333, 0.0001);
    }
}