use std::{env::args, fs};

use chumsky::{chain::Chain, prelude::*};

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    nums: Vec<usize>,
    winning_nums: Vec<usize>,
    instances: usize,
}

fn parser() -> impl Parser<char, Vec<Card>, Error = Simple<char>> {
    let digit = text::int(10)
        .padded()
        .map(|c: String| c.parse::<usize>().unwrap());

    let id = just("Card ").ignore_then(digit).then_ignore(just(':'));

    let nums = digit
        .repeated()
        .then_ignore(just('|'))
        .then(digit.repeated());

    id.then(nums)
        .map(|(id, (winning_nums, nums))| Card {
            id,
            winning_nums,
            nums,
            instances: 1,
        })
        .repeated()
}

trait Calculated {
    fn calculate1(&self) -> usize;
}

trait Scratchcards {
    // fn get_original(index: &usize) -> Option<&Card>;
    fn calculate2(&mut self) -> usize;
}

impl Card {
    fn is_winning(&self, num: &usize) -> bool {
        self.winning_nums.iter().any(|u| u == num)
    }

    fn matching_numbers(&self) -> impl Iterator<Item = &usize> {
        self.nums
            .iter()
            .filter(|item| self.winning_nums.contains(item))
    }
}

impl Calculated for Card {
    fn calculate1(&self) -> usize {
        let mut score = 0;

        for num in &self.nums {
            if !self.is_winning(num) {
                continue;
            }

            score += if score == 0 { 1 } else { score };
        }

        score
    }
}

impl Calculated for Vec<Card> {
    fn calculate1(&self) -> usize {
        self.iter().map(|c| c.calculate1()).sum()
    }
}

impl Scratchcards for Vec<Card> {
    fn calculate2(&mut self) -> usize {
        for i in 0..self.iter().count().clone() {
            let card = self.get(i).unwrap();
            let instances = card.instances;
            let winning = card.matching_numbers().count();

            println!("Card {}: Got {} matching numbers", card.id, winning);

            for ii in 0..winning {
                let dup_card = self.get_mut(1 + i + ii).unwrap();
                println!("  Copy {}", dup_card.id);
                dup_card.instances += instances;
            }
        }

        self.iter().map(|c| c.instances).sum()
    }
}

fn main() {
    let mut args = args();
    args.next();

    let path = args.next().unwrap();
    let src = fs::read_to_string(path).unwrap();

    let mut cards = parser().parse(src).unwrap();

    dbg!(cards.calculate1());
    dbg!(cards.calculate2());
}
