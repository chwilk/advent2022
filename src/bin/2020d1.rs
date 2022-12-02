use std::env;
use itertools::Itertools;
use std::fs;
use advent2021::find_filename;

fn main() {

    let filename = find_filename(env::args());

    let entries: Vec<usize> = fs::read_to_string(filename)
        .unwrap_or_else(|error| {
            panic!("Could not read file: {:?}", error);
        })
        .lines()
            .map(|entry| entry.parse::<usize>().unwrap_or(0))
        .sorted()
        .collect();
    for i in 0..entries.len() {
        match entries.binary_search(&(2020-entries[i])) {
                Ok(idx) => {
                    println! ("{}", entries[i] * entries[idx]);
                    break;
                },
                _ => (),
        }
    }
}
