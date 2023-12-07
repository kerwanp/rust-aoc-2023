use std::{cmp::Ordering, env::args, fs};

use regex::Regex;

#[derive(Debug, Clone)]
struct Game {
    cards: [char; 5],
    bid: usize,
}

fn parse(src: &str) -> Vec<Game> {
    let reg = Regex::new(r"(.+) (.+)").unwrap();

    src.split('\n')
        .filter_map(|l| reg.captures(l))
        .map(|c| (c.get(1).unwrap(), c.get(2).unwrap()))
        .map(|(a, b)| {
            Game::new(
                a.as_str()
                    .chars()
                    .take(5)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
                b.as_str().parse::<usize>().unwrap(),
            )
        })
        .collect()
}

trait Scored {
    fn score(&self) -> usize;
    fn compare(&self, with: &Self) -> Ordering;
}

static SCORES: [char; 14] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2', '1',
];

impl Scored for char {
    fn score(&self) -> usize {
        SCORES
            .iter()
            .enumerate()
            .find_map(|(pos, char)| {
                if char == self {
                    Some(SCORES.len() - pos)
                } else {
                    None
                }
            })
            .unwrap_or(0)
    }

    fn compare(&self, with: &Self) -> Ordering {
        if self.score() > with.score() {
            Ordering::Greater
        } else if self.score() < with.score() {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl Scored for [char] {
    fn score(&self) -> usize {
        let mut data = vec![];
        for card in &SCORES {
            let occ = self.iter().filter(|c| *c == card).count();

            if occ > 1 {
                data.push(occ)
            }
        }

        match (data.first(), data.get(1)) {
            (Some(5), None) => 7,
            (Some(4), None) => 6,
            (Some(3), Some(2)) | (Some(2), Some(3)) => 5,
            (Some(3), None) => 4,
            (Some(2), Some(2)) => 3,
            (Some(2), None) => 2,
            (None, None) => 1,
            (a, b) => panic!("Pattern not handled {:?} {:?}", a, b),
        }
    }

    fn compare(&self, with: &Self) -> Ordering {
        if self.score() < with.score() {
            return Ordering::Less;
        }

        if self.score() > with.score() {
            return Ordering::Greater;
        }

        for (a, b) in self.iter().zip(with.iter()) {
            let ordering = a.compare(b);

            match ordering {
                Ordering::Equal => continue,
                o => return o,
            }
        }

        Ordering::Equal
    }
}

impl Game {
    fn new(cards: [char; 5], score: usize) -> Self {
        Self { cards, bid: score }
    }
}

fn main() {
    let mut args = args();
    args.next();
    let path = args.next().unwrap();
    let src = fs::read_to_string(path).unwrap();

    let mut games = parse(&src);

    games.sort_by(|a, b| a.cards.compare(&b.cards));

    let result: usize = games
        .iter()
        .enumerate()
        .map(|(pos, game)| game.bid * (pos + 1))
        .sum();

    println!("Result: {result}");
}
