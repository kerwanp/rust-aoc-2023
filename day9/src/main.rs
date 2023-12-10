use std::{env::args, fs};

fn sequence(history: Vec<i32>) -> Vec<Vec<i32>> {
    let mut arr: Vec<Vec<i32>> = vec![history];

    loop {
        let history = arr.last().unwrap();

        if history.iter().all(|num| *num == 0) {
            break;
        }

        let differences = history
            .iter()
            .take(history.len() - 1)
            .enumerate()
            .map(|(i, num)| history[i + 1] - *num)
            .collect::<Vec<_>>();

        arr.push(differences);
    }

    arr
}

fn extrapolate_right(sequence: Vec<Vec<i32>>) -> i32 {
    let mut last = 0;

    for seq in sequence.into_iter().rev().skip(1) {
        let curr = seq.last().unwrap();
        last += curr;
    }

    last
}

fn extrapolate_left(sequence: Vec<Vec<i32>>) -> i32 {
    let mut last = 0;

    for seq in sequence.into_iter().rev().skip(1) {
        let curr = seq.first().unwrap();
        last = curr - last;
    }

    last
}

fn parse(src: &str) -> Vec<Vec<i32>> {
    src.lines()
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn sequensize_left(src: Vec<Vec<i32>>) -> i32 {
    src.into_iter().map(sequence).map(extrapolate_left).sum()
}

fn sequensize_right(src: Vec<Vec<i32>>) -> i32 {
    src.into_iter().map(sequence).map(extrapolate_right).sum()
}

fn main() {
    let mut args = args();
    args.next();
    let path = args.next().unwrap();
    let src = fs::read_to_string(path).unwrap();
    let data = parse(&src);

    let part1 = sequensize_right(data.clone());
    let part2 = sequensize_left(data);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
