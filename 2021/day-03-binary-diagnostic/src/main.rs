use std::io::{self, BufRead, Error, ErrorKind};

// While the problem can be solved with a simple for loop,
// Here I pretend to write it as a part of a large software.

fn read_diagnostic_report() -> Result<Vec<Vec<bool>>, Error> {
    let stdin = io::stdin();
    let mut v = vec![];
    for r in stdin.lock().lines() {
        let line = r?;
        let mut booleans = vec![];
        for c in line.chars() {
            let v = match c {
                '0' => Ok(false),
                '1' => Ok(true),
                _ => Err(Error::from(ErrorKind::InvalidData)),
            }?;
            booleans.push(v);
        }
        v.push(booleans);
    }
    Ok(v)
}
fn main() -> Result<(), Error> {
    let report = read_diagnostic_report()?;
    let matrix: Vec<Vec<i64>> = report
        .iter()
        .map(|v| {
            v.iter()
                .map(|e| match e {
                    true => 1,
                    false => -1,
                })
                .collect()
        })
        .collect();
    let calculated = matrix
        .iter()
        .cloned()
        .reduce(|v1, v2| v1.iter().zip(v2.iter()).map(|(&e1, &e2)| e1 + e2).collect())
        .ok_or(Error::from(ErrorKind::InvalidData))?;

    let result: Vec<bool> = calculated.iter().map(|&i| i >= 0).rev().collect();

    let gamma_rate = result
        .iter()
        .enumerate()
        .fold(0, |current, (i, &b)| current + ((b as i64) << i));
    let epsilon_rate = result
        .iter()
        .enumerate()
        .fold(0, |current, (i, &b)| current + ((!b as i64) << i));
    let power_consumption = gamma_rate * epsilon_rate;

    println!("gamma rate: {}", gamma_rate);
    println!("epsilon rate: {}", epsilon_rate);
    println!("power consumption: {}", power_consumption);

    Ok(())
}
