use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

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
