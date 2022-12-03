use std::env;
use std::fs;
use advent2022::find_filename;

fn main() {

    let filename = find_filename(env::args());

    let input = fs::read_to_string(filename)
    .unwrap_or_else(|error| {
        panic!("Could not read file: {:?}", error);
    });

    println! ("{}", parta(&input));
    println! ("{}", partb(&input));
}

fn parta (input: &String) -> u32 {
    let items: Vec<u32> = input.lines()
        .map(|sack| {
            let (a, b) = sack.split_at(sack.len()/2);
            let mut dupe: char = 'a';
            for i in a.chars() {
                if b.contains(i) {
                    dupe = i;
                    break;
                }
            }
            dupe
        })
        .map(|item| {
            match item {
                'A'..='Z' => item as u32 - 38,
                _ => item as u32 - 96,
            }
        })
        .collect();

    items.iter().sum::<u32>()
}

fn partb (input: &String) -> u32 {
    let sacks: Vec<u32> = input.lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|team| {
            let mut badge: char = 'a';
            for item in team[0].chars() {
                if team[1].contains(item) && team[2].contains(item) {
                    badge = item;
                    break;
                }
            }
            badge
        })
        .map(|item| {
            match item {
                'A'..='Z' => item as u32 - 38,
                _ => item as u32 - 96,
            }
        })
        .collect();

    sacks.iter().sum::<u32>()
}