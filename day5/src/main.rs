use std::{
    env::args,
    fs,
    ops::Range,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

use chumsky::{chain::Chain, prelude::*, Span};
use futures::future::{join_all, try_join_all};

#[derive(Debug, Clone)]
pub struct Seed(usize);

#[derive(Debug, Clone)]
pub struct MapRanges {
    destination: Range<usize>,
    source: Range<usize>,
}

#[derive(Debug, Clone)]
pub struct Map {
    destination: String,
    source: String,
    ranges: Vec<MapRanges>,
}

#[derive(Debug, Clone)]
pub struct Almanac {
    seeds: Vec<Seed>,
    maps: Vec<Map>,
}

pub fn num_parser() -> impl Parser<char, usize, Error = Simple<char>> {
    text::int(10)
        .map(|c: String| c.parse::<usize>().unwrap())
        .padded()
}

type SeedParser = dyn Parser<char, Vec<Seed>, Error = Simple<char>>;

pub fn seed_parser1() -> impl Parser<char, Vec<Seed>, Error = Simple<char>> {
    just("seeds:").ignore_then(num_parser().map(Seed).repeated())
}

pub fn seed_parser2() -> impl Parser<char, Vec<Seed>, Error = Simple<char>> {
    just("seeds:").ignore_then(
        num_parser()
            .then(num_parser())
            .map(|(from, to)| (from..from + to).map(Seed).collect::<Vec<_>>())
            .repeated()
            .flatten(),
    )
}

pub fn map_parser() -> impl Parser<char, Map, Error = Simple<char>> {
    let ident = text::ident();

    let range = num_parser()
        .then(num_parser())
        .then(num_parser())
        .map(|((d, s), l)| MapRanges {
            destination: d..d + l,
            source: s..s + l,
        });

    ident
        .then_ignore(just("-to-"))
        .then(ident)
        .then_ignore(just(" map:"))
        .then(range.repeated())
        .map(|((source, destination), ranges)| Map {
            source,
            destination,
            ranges,
        })
}

pub fn parser1() -> impl Parser<char, Almanac, Error = Simple<char>> {
    seed_parser1()
        .then(map_parser().repeated())
        .map(|(seeds, maps)| Almanac { seeds, maps })
}

pub fn parser2() -> impl Parser<char, Almanac, Error = Simple<char>> {
    seed_parser2()
        .then(map_parser().repeated())
        .map(|(seeds, maps)| Almanac { seeds, maps })
}

trait VecMapRanges {
    fn find_map_range(&self, num: &usize) -> Option<&MapRanges>;
}

impl VecMapRanges for Vec<MapRanges> {
    fn find_map_range(&self, num: &usize) -> Option<&MapRanges> {
        self.iter().find(|mr| mr.source.contains(num))
    }
}

impl Almanac {
    fn calculate_seed(&self, seed: &Seed) -> usize {
        let mut ids = vec![seed.0];

        for map in &self.maps {
            // println!("  map {}-to-{}", map.source, map.destination);
            let id = ids.last().unwrap();
            // println!("    input: {}", id);
            let mr = map.ranges.find_map_range(id);
            // println!("    range: {:?}", mr);

            let next_id = match mr {
                Some(mr) => mr.destination.start() + id - mr.source.start(),
                None => *id,
            };

            // println!("    output: {}", next_id);

            ids.push(next_id);
        }

        *ids.last().unwrap()
    }

    pub async fn calculate(&self) -> usize {
        0
        // let mut output = vec![];
        //
        // for chunk in self.seeds.chunks(32) {
        //     output.push(tokio::spawn(async move {
        //         chunk
        //             .iter()
        //             .map(|c| self.calculate_seed(c))
        //             .collect::<Vec<_>>()
        //     }));
        // }
        //
        // let res = try_join_all(output).await;
        //
        // *res.iter().flatten().flatten().min().unwrap()
    }
}

pub async fn calculate(almanac: Almanac) -> usize {
    let mut ts = vec![];

    let almanac = Arc::new(almanac);

    for i in 0..almanac.seeds.chunks(almanac.seeds.len() / 16).count() {
        let almanac = almanac.clone();
        println!("Chunk {i}");
        ts.push(tokio::spawn(async move {
            for seed in almanac.seeds.chunks(almanac.seeds.len() / 16).nth(i) {}
            almanac
                .seeds
                .chunks(16)
                .nth(i)
                .unwrap_or(&[])
                .iter()
                .map(|seed| almanac.calculate_seed(seed))
                .collect::<Vec<_>>()
        }));
    }

    *join_all(ts).await.iter().flatten().flatten().min().unwrap()
}

#[tokio::main]
async fn main() {
    let mut args = args();
    args.next();

    let path = args.next().unwrap();
    let src = fs::read_to_string(path).unwrap();

    let part1 = parser1().parse(src.clone()).unwrap();
    println!("Part1: {}", calculate(part1).await);
    let part2 = parser2().parse(src).unwrap();

    println!("Part2: {}", calculate(part2).await);
}
