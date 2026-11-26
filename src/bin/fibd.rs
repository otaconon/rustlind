use std::io::Write;

use anyhow::Result;
use rustlind::{FileIO, Scanner};

fn fib_dp(sim_time: usize, life_time: usize) -> u128 {
    let mut adults_at_t = vec![0; sim_time];
    let mut born_at_t = vec![0; sim_time];
    born_at_t[0] = 1;
    for t in 1..=sim_time-1 {
        adults_at_t[t] = adults_at_t[t-1] + born_at_t[t-1];
        if t >= life_time {
            adults_at_t[t] -= born_at_t[t-life_time];
        }
        born_at_t[t] = adults_at_t[t-1];
    }
    adults_at_t[sim_time - 1] + born_at_t[sim_time-1]
}

fn main() -> Result<()> {
    let mut io = FileIO::from_args()?;
    let mut scanner = Scanner::new(io.reader);
    let sim_time = scanner.next::<u32>()?;
    let life_time = scanner.next::<u32>()?;
    writeln!(io.writer, "{}", fib_dp(sim_time as usize, life_time as usize))?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fib_dp() {
        let sim_time = 6;
        let life_time = 3;
        assert_eq!(fib_dp(sim_time, life_time), 4);
    }
}
