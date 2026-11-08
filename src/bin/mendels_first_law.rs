use std::io::{Write};
use std::error::Error;

use rustlind::{FileIO, Scanner};

fn dominant_allele_prob(k: u32, m: u32, n: u32) -> f64 {
    let (k, m, n) = (k as f64, m as f64, n as f64);
    let t = k+m+n;
    1.0 - (n*(n-1.0) + m*n + m*(m-1.0)*0.25) / (t*(t-1.0))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut io = FileIO::from_args()?;
    let mut scanner = Scanner::new(io.reader);
    let k = scanner.next::<u32>()?;
    let m = scanner.next::<u32>()?;
    let n = scanner.next::<u32>()?;
    writeln!(io.writer, "{:.5}", dominant_allele_prob(k, m, n))?;
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
        let (k, m, n) = (2, 2, 2);
        assert_delta!(dominant_allele_prob(k, m, n), 0.78333, 0.0001);
    }
}