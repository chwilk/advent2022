use std::env;
use itertools::Itertools;
use std::fs;
use advent2021::find_filename;

fn main() {

    let filename = find_filename(env::args());

    let mut elves: Vec<u32> = fs::read_to_string(filename)
        .unwrap_or_else(|error| {
            panic!("Could not read file: {:?}", error);
        })
        .split("\n\n")
        .map(|i| {
            i.split('\n')
                .map(|line| line.parse::<u32>().unwrap_or(0)).sum()
        })
        .sorted()
        .rev()
        .collect();

    println! ("{}", elves[0]);
    println! ("{}", elves.iter().take(3).sum::<u32>());
}
