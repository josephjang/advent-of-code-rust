use std::io;
use std::io::{BufRead, Error, ErrorKind};

fn read_measurements() -> Result<Vec<i64>, Error> {
    let stdin = io::stdin();
    let mut v = vec![];
    for r in stdin.lock().lines() {
        v.push(
            r?.trim()
                .parse()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?,
        );
    }
    Ok(v)
}

fn main() -> Result<(), Error> {
    let measurements = read_measurements()?;

    // Part One

    let mut count: i64 = 0;
    let mut prev: Option<i64> = None;

    for m in &measurements {
        match prev {
            Some(pm) => {
                if m > &pm {
                    count += 1;
                }
            }
            None => {}
        }
        prev = Some(*m);
    }

    println!("{}", count);

    // Part Two

    let mut count: i64 = 0;
    let mut prev: Option<i64> = None;
    let len = measurements.len();

    for i in 0..len - 2 {
        let sum = measurements[i] + measurements[i + 1] + measurements[i + 2];

        match prev {
            Some(pm) => {
                if sum > pm {
                    count += 1;
                }
            }
            None => {}
        }
        prev = Some(sum);
    }

    println!("{}", count);

    Ok(())
}
