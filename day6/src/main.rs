use std::{env::args, fs};

use chumsky::prelude::*;

#[derive(Debug, Clone)]
struct Race {
    time: usize,
    record: usize,
}

fn parser1() -> impl Parser<char, Vec<Race>, Error = Simple<char>> {
    let num = text::int(10)
        .map(|s: String| s.parse::<usize>().unwrap())
        .padded();

    let times = just("Time:").ignore_then(num.repeated());
    let distances = just("Distance:").ignore_then(num.repeated());

    times.then(distances).map(|(times, distances)| {
        times
            .iter()
            .zip(distances)
            .map(|(time, distance)| Race {
                time: *time,
                record: distance,
            })
            .collect()
    })
}

fn parser2() -> impl Parser<char, Vec<Race>, Error = Simple<char>> {
    let num = text::int(10)
        .padded()
        .repeated()
        .map(|n| n.join("").parse::<usize>().unwrap());

    let times = just("Time:").ignore_then(num);
    let distances = just("Distance:").ignore_then(num);

    times
        .then(distances)
        .map(|(time, record)| Race { time, record })
        .repeated()
}

impl Race {
    pub fn possibilities(&self) -> usize {
        println!("Race {self:?}");

        (0..self.time)
            .filter(|hold_time| {
                let travel_time = self.time - hold_time;
                let distance = travel_time * hold_time;

                distance > self.record
            })
            .count()
    }
}

fn calculate_file(path: &str) -> usize {
    let src = fs::read_to_string(path).unwrap();
    let races = parser2().parse(src).unwrap();

    races
        .iter()
        .map(|r| r.possibilities())
        .reduce(|a, b| a * b)
        .unwrap()
}

fn main() {
    let mut args = args();
    args.next();
    let path = args.next().unwrap();

    let res = calculate_file(&path);
    println!("Result: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(calculate_file("test1.txt"), 288);
    }
}
