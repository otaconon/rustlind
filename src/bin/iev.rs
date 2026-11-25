use std::io::Write;

use anyhow::{Result};

use rustlind::{FileIO, Scanner};

fn expected_offspring(genotypes_cnt: Vec<u32>) -> f64 {
  let mut expected: f64 = 0.0;
  for (idx, &cnt) in genotypes_cnt.iter().enumerate() {
    expected += (cnt as f64) * match idx {
      0 | 1 | 2 => 2.0,
      3 => 2.0*0.75,
      4 => 1.0,
      5 => 0.0,
      _ => 0.0
    }
  }
  return expected
}

fn main() -> Result<()>{
  let mut io = FileIO::from_args()?;
  let mut scanner = Scanner::new(io.reader);
  let mut genotypes_cnt = vec![0; 6];
  for i in 0..6 {
    genotypes_cnt[i] = scanner.next::<u32>()?;
  }
  writeln!(io.writer, "{}", expected_offspring(genotypes_cnt))?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_expected_offspring() {
    let genotypes_cnt = vec![1, 0, 0, 1, 0, 1];
    rustlind::assert_delta!(expected_offspring(genotypes_cnt), 3.5, 1e-6);
  }
}