use std::env;
use std::fs;
use regex::Regex;
use advent2022::find_filename;

fn main() {

    let filename = find_filename(env::args());

    let input = fs::read_to_string(filename)
        .unwrap_or_else(|error| {
            panic!("Could not read file: {:?}", error);
        });
    let (rstacks, moves) = input.split_once("\n\n").unwrap();

    let mut stacks = rstacks.lines().rev();

    let whitespace = Regex::new(r"[ ]+").unwrap();
    let cranes: usize = whitespace
        .split(stacks.next().unwrap())
        .into_iter()
        .filter_map(|num| num.parse::<usize>().ok())
        .collect::<Vec<usize>>()
        .len();

    let mut stacka: [Vec<char>; 9] = Default::default();
    let mut stackb: [Vec<char>; 9] = Default::default();

    for line in stacks {
        let width = (line.len() + 1) / 4;
        for s in 0..width {
            let c = line.chars().nth(s*4+1).unwrap_or('*');
            match c {
                'A'..='Z' => {
                    stacka[s].push(c);
                    stackb[s].push(c);
                },
                _ => ()
            }
        }
    }

    let crane_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for m in crane_re.captures_iter(moves) {
        let num = m[1].parse::<usize>().unwrap();
        let from = &m[2].parse::<usize>().unwrap()-1;
        let to = &m[3].parse::<usize>().unwrap()-1;
        let mut temp = Vec::new();
        for _i in 0..num {
            temp.push(stacka[from].pop().unwrap_or('*'));
            stacka[to].push(temp.pop().unwrap_or('*'));
        }
        for _i in 0..num {
            temp.push(stackb[from].pop().unwrap_or('*'));
        }
        for _i in 0..num {
            stackb[to].push(temp.pop().unwrap_or('*'));
        }
    }

    for top in 0..cranes {
        print!("{}", stacka[top].pop().unwrap_or(' '));
    }
    print!("\n");
    for top in 0..cranes {
        print!("{}", stackb[top].pop().unwrap_or(' '));
    }
    print!("\n");
}
