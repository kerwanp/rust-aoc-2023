use std::{cmp::Ordering, env::args, fs};

use regex::Regex;

#[derive(Debug, Clone)]
struct Game {
    hand: [char; 5],
    bid: usize,
}

#[derive(Debug, Clone)]
struct Cards([char; 5]);

#[derive(Debug, Clone)]
struct Hand {
    cards: Cards,
    best: [char; 5],
    score: usize,
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

trait JokerScored {
    fn joker_score(&self) -> usize;
}

static CARDS: [char; 14] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', '1', 'J',
];

impl Scored for char {
    fn score(&self) -> usize {
        CARDS
            .iter()
            .enumerate()
            .find_map(|(pos, char)| {
                if char == self {
                    Some(CARDS.len() - pos)
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

impl Scored for [char; 5] {
    fn score(&self) -> usize {
        let mut data = vec![];
        for card in &CARDS {
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
        if self.joker_score() < with.joker_score() {
            return Ordering::Less;
        }

        if self.joker_score() > with.joker_score() {
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

impl JokerScored for [char; 5] {
    fn joker_score(&self) -> usize {
        let mut hand = *self;
        let jokers = self
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if *c == 'J' { Some(i) } else { None })
            .collect::<Vec<_>>();

        let mut score = self.score();

        for card in CARDS {
            for joker in &jokers {
                hand[*joker] = card;
            }

            let new_score = hand.score();
            if new_score > score {
                score = new_score;
                println!(" Best {hand:?}");
            }
        }

        score
    }
}

impl Game {
    fn new(cards: [char; 5], score: usize) -> Self {
        Self {
            hand: cards,
            bid: score,
        }
    }
}

fn main() {
    let mut args = args();
    args.next();
    let path = args.next().unwrap();
    let src = fs::read_to_string(path).unwrap();

    let mut games = parse(&src);

    games.sort_by(|a, b| a.hand.compare(&b.hand));

    // dbg!(&games);

    let result: usize = games
        .iter()
        .enumerate()
        .map(|(pos, game)| game.bid * (pos + 1))
        .sum();

    println!("Result: {result}");
}

impl Cards {}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
