use std::env;
use std::fs;
use advent2022::find_filename;

#[derive(Debug)]
struct Pair {
    r1: std::ops::Range<usize>,
    r2: std::ops::Range<usize>,
}

fn main() {

    let filename = find_filename(env::args());

    let input = fs::read_to_string(filename)
    .unwrap_or_else(|error| {
        panic!("Could not read file: {:?}", error);
    })
    .lines()
    .map(|line| {
        let mut ranges = line.split(',')
            .into_iter()
            .map(|range| {
                let mut indices = range
                    .split('-')
                    .map(|idx| idx.parse::<usize>().unwrap_or(0))
                    .into_iter();
                let i1 = indices.next().unwrap_or(0);
                let i2 = indices.next().unwrap_or(0);
                i1..i2
            })
            .into_iter();
        Pair {
            r1: ranges.next().unwrap(),
            r2: ranges.next().unwrap(),
        }
    })
    .collect();

    println! ("{}", parta(&input));
    println! ("{}", partb(&input));
}

fn parta (input: &Vec<Pair>) -> usize {
    input
        .iter()
        .map(|pair| {
            if pair.r1.start < pair.r2.start {
                if pair.r1.end >= pair.r2.end { 1 }
                else { 0 }
            } else if pair.r1.start > pair.r2.start {
                if pair.r1.end <= pair.r2.end { 1 }
                else { 0 }
            } else {1}
        })
        .sum()
}

fn partb (input: &Vec<Pair>) -> usize {
    input
        .iter()
        .map(|pair| {
            if pair.r1.end < pair.r2.start {
                0
            } else if pair.r2.end < pair.r1.start {
                0
            } else { 
                1
            }
        })
        .sum()
}