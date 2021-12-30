use std::io::{self, BufRead, Error, ErrorKind};

enum Orientation {
    Forward,
    Up,
    Down,
}

struct Command {
    orientation: Orientation,
    distance: i64,
}

fn read_commands() -> Result<Vec<Command>, Error> {
    let stdin = io::stdin();
    let mut v = vec![];
    for r in stdin.lock().lines() {
        let line = r?;
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let orientation = match tokens[0] {
            "forward" => Ok(Orientation::Forward),
            "up" => Ok(Orientation::Up),
            "down" => Ok(Orientation::Down),
            _ => Err(Error::from(ErrorKind::InvalidData)),
        }?;
        let distance: i64 = tokens[1]
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        v.push(Command {
            orientation,
            distance,
        });
    }
    Ok(v)
}

fn main() -> Result<(), Error> {
    let commands = read_commands()?;

    // Part One

    let mut position: i64 = 0;
    let mut depth: i64 = 0;

    for command in &commands {
        match command {
            Command {
                orientation: Orientation::Forward,
                distance,
            } => position += distance,
            Command {
                orientation: Orientation::Up,
                distance,
            } => depth -= distance,
            Command {
                orientation: Orientation::Down,
                distance,
            } => depth += distance,
        }
    }

    println!("position: {}", position);
    println!("depth: {}", depth);
    println!("position x depth: {}", position * depth);

    // Part Two

    let mut position: i64 = 0;
    let mut depth: i64 = 0;
    let mut aim: i64 = 0;

    for command in &commands {
        match command {
            Command {
                orientation: Orientation::Forward,
                distance,
            } => {
                position += distance;
                depth += aim * distance;
            }
            Command {
                orientation: Orientation::Up,
                distance,
            } => aim -= distance,
            Command {
                orientation: Orientation::Down,
                distance,
            } => aim += distance,
        }
    }

    println!("position: {}", position);
    println!("depth: {}", depth);
    println!("position x depth: {}", position * depth);

    Ok(())
}
