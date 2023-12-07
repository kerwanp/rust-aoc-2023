use std::{collections::HashMap, fs};

fn main() {
    println!("Hello, world!");

    let content = fs::read_to_string("./input.txt").unwrap();

    let lines: Vec<_> = content.split('\n').collect();
    let res = calibrate(lines);
    println!("OOHOHOHO {}", res);
}

fn calibrate(lines: Vec<&str>) -> usize {
    let mut result = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        result += calibrate_line(line);
    }
    result
}

fn find_num(input: &str) -> (usize, usize) {
    let mut map: HashMap<&str, usize> = HashMap::new();
    map.insert("zero", 0);
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);
    map.insert("0", 0);
    map.insert("1", 1);
    map.insert("2", 2);
    map.insert("3", 3);
    map.insert("4", 4);
    map.insert("5", 5);
    map.insert("6", 6);
    map.insert("7", 7);
    map.insert("8", 8);
    map.insert("9", 9);

    let mut found = vec![];

    for (keyword, num) in map {
        let pos = input.match_indices(keyword).collect::<Vec<_>>();
        let val = pos.into_iter().map(|(i, _)| (i, num));
        found.extend(val);
    }

    found.sort_by(|(a, _), (b, _)| a.cmp(b));

    (found.first().unwrap().1, found.last().unwrap().1)
}

fn calibrate_line(input: &str) -> usize {
    let (a, b) = find_num(input);
    format!("{a}{b}").parse::<usize>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibrate() {
        // assert_eq!(calibrate(vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f"]), 65);
    }

    #[test]
    fn test_calibrate_line() {
        assert_eq!(calibrate_line("1abc2"), 12);
        assert_eq!(calibrate_line("pqr3stu8vwx"), 38);
        assert_eq!(calibrate_line("a1b2c3d4e5f"), 15);
        assert_eq!(calibrate_line("treb7uchet"), 77);
        assert_eq!(calibrate_line("two1nine"), 29);
        assert_eq!(calibrate_line("eighttwothree"), 83);
        assert_eq!(calibrate_line("abcone2threexyz"), 13);
        assert_eq!(calibrate_line("xtwone3four"), 24);
        assert_eq!(calibrate_line("4nineeightseven2"), 42);
        assert_eq!(calibrate_line("zoneight234"), 14);
        assert_eq!(calibrate_line("7pqrstsixteen"), 76);
        assert_eq!(calibrate_line("2fednineight"), 28);
        assert_eq!(calibrate_line("fivek5mfzrdxfbn66nine8eight"), 58);
        assert_eq!(calibrate_line("qvjqtqffvtp2six"), 26);
        assert_eq!(calibrate_line("118"), 18);
        assert_eq!(calibrate_line("5xjc"), 55);
        assert_eq!(calibrate_line("33nine"), 39);
        assert_eq!(calibrate_line("one8six6"), 16);
        assert_eq!(
            calibrate_line("seveneightmgqfcfczxsthrxhq3zcthsrxshddnlxronekdhqmmbhzd"),
            71
        );
        assert_eq!(calibrate_line("8"), 88);
        assert_eq!(calibrate_line("eight"), 88);
        assert_eq!(calibrate_line("2513"), 23);
        assert_eq!(calibrate_line("a11a"), 11);
        assert_eq!(calibrate_line("aoneonea"), 11);
        assert_eq!(calibrate_line("11"), 11);
        assert_eq!(calibrate_line("oneone"), 11);
        assert_eq!(calibrate_line("nineight"), 98);
        assert_eq!(calibrate_line("1aaaa"), 11);
        assert_eq!(calibrate_line("aaaa1"), 11);
        assert_eq!(calibrate_line("onetwothreetwoone"), 11);
    }
}
