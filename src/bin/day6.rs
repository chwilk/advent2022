use std::env;
use std::fs;
use advent2022::find_filename;

struct Part {
    a: usize,
    b: usize,
}

fn main() {

    let filename = find_filename(env::args());

    let sig: Vec<Part> = fs::read_to_string(filename)
        .unwrap_or_else(|error| {
            panic!("Could not read file: {:?}", error);
        })
        .lines()
        .map(|l| {
            let c:Vec<char> = l.chars().collect();
            let mut a = 3;
            let mut b = 13;
            for i in a..(c.len()) {
                if c[i-3] != c[i-2] &&
                   c[i-3] != c[i-1] &&
                   c[i-3] != c[i] &&
                   c[i-2] != c[i-1] &&
                   c[i-2] != c[i] &&
                   c[i-1] != c[i] {
                    a = i;
                    break;
                }
            } 
            for i in b..(c.len()) {
                let mut test: [char; 14] = Default::default();
                test.clone_from_slice(&c[(i-13)..i+1]);
                test.sort();
                let mut dupe = false;
                for j in 0..13 {
                    if test[j] == test[j+1] {
                        dupe = true;
                        break;
                    }
                }
                if dupe == false {
                    b = i;
                    break;
                }    
            }
            Part {
                a,
                b,
            }
        })
        .collect();

    for i in sig {
        println!("{} {}", i.a+1, i.b+1);
    }
}
