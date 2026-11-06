use std::io::{Write};
use std::error::Error;

use rustlind::{FileIO, Strand, StrandType};

fn dominant_allele_prob(kmn: &[u32; 3]) -> f64 {
    let [k, m, n]: [f64; 3] = (*kmn).map(|x| x as f64);
    let t = k+m+n;
    1.0 - (n*(n-1.0) + m*n + m*(m-1.0)*0.25) / (t*(t-1.0))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut io = FileIO::from_args()?;
    let raw_input = io.read()?;
    let kmn: [u32; 3] = std::str::from_utf8(&raw_input)?
        .split_whitespace()
        .map(str::parse::<u32>)
        .collect::<Result<Vec<_>, _>>()?
        .try_into()
        .map_err(|v: Vec<u32>| format!("expected 3 integers, got {}", v.len()))?;
    writeln!(io.writer, "{:.5}", dominant_allele_prob(&kmn))?;
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
        let kmn: [u32; 3] = [2, 2, 2];
        assert_delta!(dominant_allele_prob(&kmn), 0.78333, 0.0001);
    }
}