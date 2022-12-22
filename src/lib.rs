//use std::error::Error;
use std::env;
use std::format;

pub fn find_filename(mut args: env::Args) -> String {
    let filename: String;
    if args.len() < 2 { // Try to guess a test#.dat from binary name
        let day = parse_int(args.nth(0).unwrap()).unwrap();
        filename = format!("tests/inputs/test{}.dat", day);
    } else {
        filename = args.nth(1).unwrap().clone();
    }
    filename
}

fn parse_int(input: String) -> Option<u32> {
    input
        .chars()
        .skip_while(|ch| !ch.is_digit(10))
        .take_while(|ch| ch.is_digit(10))
        .fold(None, |acc, ch| {
            ch.to_digit(10).map(|b| acc.unwrap_or(0) * 10 + b)
        })
}

pub fn alphabetize(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.into_iter().collect::<String>()
}

