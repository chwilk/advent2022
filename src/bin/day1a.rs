use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("File error");

    let mut max = 0;
    let mut current = 0;

    for line in contents.lines() {
        if line == "" {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            current = current + line.parse::<i32>().unwrap();
        }
    }
    if current > max {
        max = current;
    }

    println! ("{}", max);
}
