use std::env;
use std::fs;
use advent2022::find_filename;

fn main() {

    let filename = find_filename(env::args());

    let sig: Vec<usize> = fs::read_to_string(filename)
        .unwrap_or_else(|error| {
            panic!("Could not read file: {:?}", error);
        })
        .lines()
        .map(|l| {
            let c:Vec<char> = l.chars().collect();
            let mut res = 0;
            for i in 3..(c.len()) {
                if c[i-3] != c[i-2] &&
                   c[i-3] != c[i-1] &&
                   c[i-3] != c[i] &&
                   c[i-2] != c[i-1] &&
                   c[i-2] != c[i] &&
                   c[i-1] != c[i] {
                    res = i;
                    break
                }
            } 
            res
        })
        .collect();

    for i in sig {
        println!("{}", i+1);
    }
}
