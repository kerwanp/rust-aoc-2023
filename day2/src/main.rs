use std::{collections::HashMap, env::args, fs};

use chumsky::prelude::*;
use std::iter::Iterator;

#[derive(Debug, PartialEq, Eq)]
pub struct Cube(u32, String);
#[derive(Debug, PartialEq, Eq)]
pub struct CubeSet(Vec<Cube>);
#[derive(Debug, PartialEq, Eq)]
pub struct Game(u32, Vec<CubeSet>);

fn num_parser() -> impl Parser<char, u32, Error = Simple<char>> {
    text::digits(10)
        .map(|d: String| d.parse().unwrap())
        .padded()
}

fn cube_parser() -> impl Parser<char, Cube, Error = Simple<char>> {
    let color = text::ident().padded();

    num_parser()
        .then(color)
        .map(|(num, color)| Cube(num, color))
}

fn cubeset_parser() -> impl Parser<char, CubeSet, Error = Simple<char>> {
    cube_parser().separated_by(just(',')).map(CubeSet)
}

fn game_parser() -> impl Parser<char, Game, Error = Simple<char>> {
    let cubesets = cubeset_parser().separated_by(just(';'));
    let game_id = just("Game")
        .ignore_then(num_parser())
        .then_ignore(just(':'));

    game_id
        .then(cubesets)
        .map(|(id, cubesets)| Game(id, cubesets))
}

pub fn minimum_cubes(Game(.., cubesets): &Game, color: &str) -> u32 {
    let mut min: u32 = 1;
    for CubeSet(cubeset) in cubesets {
        let count = cubeset.iter().find(|c| c.1.as_str() == color);

        let Some(Cube(count, ..)) = count else {
            continue;
        };

        if *count > min {
            min = *count;
        }
    }

    min
}

pub fn is_game_possible(Game(.., cubesets): &Game) -> bool {
    let mut hashmap: HashMap<&str, u32> = HashMap::new();
    hashmap.insert("red", 12);
    hashmap.insert("green", 13);
    hashmap.insert("blue", 14);

    for CubeSet(cubes) in cubesets {
        for Cube(num, color) in cubes {
            let contained = hashmap.get(color.as_str()).unwrap();
            if num > contained {
                return false;
            }
        }
    }

    true
}

fn main() {
    let mut args = args();
    args.next();

    let content = fs::read_to_string(args.next().expect("file name")).expect("file content");

    let mut possible_games = 0;
    let mut power_sum = 0;
    for line in content.split('\n') {
        if line.is_empty() {
            continue;
        }

        let game = game_parser().parse(line).unwrap();
        if is_game_possible(&game) {
            possible_games += game.0;
        }

        let min_red = minimum_cubes(&game, "red");
        let min_blue = minimum_cubes(&game, "blue");
        let min_green = minimum_cubes(&game, "green");
        let power = min_red * min_blue * min_green;

        println!("Game {}", game.0);
        println!("  Red: {min_red}, Blue: {min_blue}, Green: {min_green}");
        println!("  Power: {power}");

        power_sum += min_red * min_blue * min_green;
    }

    println!("Possible games: {}", possible_games);
    println!("Power: {}", power_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_minimum_cubes() {
        let game = Game(
            1,
            vec![
                CubeSet(vec![
                    Cube(3, "blue".to_string()),
                    Cube(4, "red".to_string()),
                ]),
                CubeSet(vec![
                    Cube(1, "red".to_string()),
                    Cube(2, "green".to_string()),
                    Cube(6, "blue".to_string()),
                ]),
                CubeSet(vec![Cube(2, "green".to_string())]),
            ],
        );

        assert_eq!(minimum_cubes(&game, "blue"), 3);
        assert_eq!(minimum_cubes(&game, "red"), 1);
        assert_eq!(minimum_cubes(&game, "green"), 2);
    }

    #[test]
    pub fn test_parse_cube() {
        assert_eq!(
            cube_parser().parse("5 blue").unwrap(),
            Cube(5, "blue".to_string())
        );
    }

    #[test]
    pub fn test_parse_cubeset() {
        assert_eq!(
            cubeset_parser().parse("5 blue, 1 red, 2 green").unwrap(),
            CubeSet(vec![
                Cube(5, "blue".to_string()),
                Cube(1, "red".to_string()),
                Cube(2, "green".to_string())
            ])
        );
    }

    #[test]
    pub fn test_parse_game() {
        assert_eq!(
            game_parser()
                .parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
                .unwrap(),
            Game(
                1,
                vec![
                    CubeSet(vec![
                        Cube(3, "blue".to_string()),
                        Cube(4, "red".to_string()),
                    ]),
                    CubeSet(vec![
                        Cube(1, "red".to_string()),
                        Cube(2, "green".to_string()),
                        Cube(6, "blue".to_string()),
                    ]),
                    CubeSet(vec![Cube(2, "green".to_string()),])
                ]
            )
        );
    }
}
