use std::{collections::HashMap, env::args, fs, usize};

use regex::Regex;

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Position(String, String);

fn parse(src: &str) -> (Vec<Direction>, HashMap<String, Position>) {
    let re = Regex::new(r"(.{3}) = \((.{3}), (.{3})\)").unwrap();
    let lines = src.split('\n').collect::<Vec<_>>();

    let instructions = lines
        .first()
        .unwrap()
        .chars()
        .filter_map(|c| match c {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        })
        .collect::<Vec<_>>();

    let positions = lines
        .iter()
        .skip(2)
        .filter_map(|line| re.captures(line))
        .map(|c| {
            (
                c.get(1).unwrap().as_str().to_string(),
                Position(
                    c.get(2).unwrap().as_str().to_string(),
                    c.get(3).unwrap().as_str().to_string(),
                ),
            )
        })
        .collect::<HashMap<_, _>>();

    (instructions, positions)
}

fn navigate(instructions: Vec<Direction>, map: HashMap<String, Position>) -> usize {
    let positions = map
        .keys()
        .filter(|pos| pos.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();

    let mut iterations: Vec<usize> = vec![];

    for mut position in &positions {
        let mut count = 0;
        let mut i = 0;
        while !position.ends_with('Z') {
            let instruction = instructions.get(i).unwrap();

            let Position(left, right) = map.get(position).unwrap();

            position = match instruction {
                Direction::Left => left,
                Direction::Right => right,
            };

            count += 1;
            if i >= instructions.len() - 1 {
                i = 0;
            } else {
                i += 1;
            }
        }

        iterations.push(count);
        println!("Found {count} iterations for pos {position:?}");
    }

    lcm(&iterations)
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }

    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(b, a % b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn main() {
    let mut args = args();
    args.next();
    let path = args.next().unwrap();
    let src = fs::read_to_string(path).unwrap();
    let (instructions, map) = parse(&src);
    let result = navigate(instructions, map);
    println!("Result: {result}");
}
