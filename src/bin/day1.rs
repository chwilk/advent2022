use std::env;
use std::fs;
use advent2021::find_filename;

fn main() {

    let filename = find_filename(env::args());

    let contents = fs::read_to_string(filename).expect("File error");

    let mut elves =  Vec::new();
    let mut current = 0;

    for line in contents.lines() {
        if line == "" {
            elves.push(current);
            current = 0;
        } else {
            current = current + line.parse::<i32>().unwrap();
        }
    }
    elves.push(current);

    elves.sort();

    let mut sum = elves.pop().unwrap_or(0);
    sum = sum + elves.pop().unwrap_or(0);
    sum = sum + elves.pop().unwrap_or(0);

    println! ("{}", sum);
}
