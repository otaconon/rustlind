pub mod fasta;
mod file_io;
mod nucleotide;
mod scanner;
mod strand;
mod strand_type;
mod codon;
mod protein;
mod amino_acid;

pub use file_io::FileIO;
pub use nucleotide::{Nucleobase, NucleobaseMap};
pub use scanner::Scanner;
pub use strand::Strand;
pub use strand_type::StrandType;
pub use codon::{Codon, CodonMap};
pub use amino_acid::AminoAcid;
pub use protein::Protein;

#[macro_export]
macro_rules! assert_delta {
    ($x:expr, $y:expr, $d:expr) => {{
        let (x, y, d): (f64, f64, f64) = ($x, $y, $d);
        assert!((x - y).abs() < d, "{x} is not within {d} of {y}");
    }};
}
