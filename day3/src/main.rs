use std::{env::args, fs, ops::RangeBounds};

#[derive(Debug, Clone)]
enum TokenKind {
    Number(usize),
    Digit(char),
    Dot,
    Gear,
    Unknown,
}

#[derive(Debug, Clone)]
struct Token {
    line: usize,
    start: usize,
    end: usize,
    kind: TokenKind,
}

fn parse(src: &str) -> Vec<Token> {
    let mut tokens = vec![];

    let mut line = 0;

    // For calculating digits coordinates
    let mut digits = vec![];
    let mut start = None;
    let mut pos = 0;

    for char in src.chars() {
        let kind = match char {
            '.' => TokenKind::Dot,
            '*' => TokenKind::Gear,
            c if c.is_ascii_digit() => TokenKind::Digit(c),
            _ => TokenKind::Unknown,
        };

        if let TokenKind::Digit(c) = kind {
            if digits.is_empty() {
                start = Some(pos);
            }

            digits.push(c);
        } else if !digits.is_empty() {
            let num = String::from_iter(&digits).parse::<usize>().unwrap();
            tokens.push(Token {
                line,
                start: start.unwrap(),
                end: pos - 1,
                kind: TokenKind::Number(num),
            });
            digits = vec![];
        }

        match kind {
            TokenKind::Digit(_) => (),
            kind => tokens.push(Token {
                line,
                start: pos,
                end: pos,
                kind,
            }),
        }

        if char == '\n' {
            line += 1;
            pos = 0;
        } else {
            pos += 1;
        }
    }

    tokens
}

fn calculate_gear(tokens: &[Token], gear: &Token) -> Option<usize> {
    let mut found = vec![];

    for line in gear.line - 1..gear.line + 2 {
        for pos in gear.start - 1..gear.start + 2 {
            let res = tokens
                .iter()
                .filter(|tok| matches!(tok.kind, TokenKind::Number(_)))
                .find(|tok| tok.line == line && (tok.start..tok.end + 1).contains(&pos));

            if let Some(res) = res {
                found.push(res);
            }
        }
    }

    found.dedup_by(|a, b| {
        if let (TokenKind::Number(num_a), TokenKind::Number(num_b)) = (&a.kind, &b.kind) {
            return num_a == num_b && a.start == b.start && a.end == b.end && a.line == b.line;
        }

        false
    });

    if found.len() != 2 {
        return None;
    }

    let toks = found
        .iter()
        .map(|t| match t.kind {
            TokenKind::Number(num) => num,
            _ => 0,
        })
        .collect::<Vec<_>>();

    Some(toks.first().unwrap() * toks.last().unwrap())
}

fn calculate(tokens: Vec<Token>) -> usize {
    tokens
        .iter()
        .filter(|tok| matches!(tok.kind, TokenKind::Gear))
        .filter_map(|t| calculate_gear(&tokens, t))
        .sum()
}

fn main() {
    let mut args = args();
    args.next();
    let path = args.next().unwrap();
    let src = fs::read_to_string(path).unwrap();
    let tokens = parse(&src);
    let res = calculate(tokens);

    dbg!(res);
}
