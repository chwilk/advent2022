use std::env;
use std::fs;
use advent2022::find_filename;

fn main() {

    let filename = find_filename(env::args());

    let items: Vec<u32> = fs::read_to_string(filename)
        .unwrap_or_else(|error| {
            panic!("Could not read file: {:?}", error);
        })
        .lines()
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

    println! ("{:?}", items);
    println! ("{:?}", items.iter().sum::<u32>());
}
