use std::collections::VecDeque;
use std::env;
use std::fs;
use itertools::Itertools;
use regex::Regex;
use advent2022::find_filename;

fn main() {

    let filename = find_filename(env::args());
    let monkey_index = Regex::new(r"Monkey (\d+):").unwrap();

    let monkeys: Vec<Monkey> = fs::read_to_string(filename)
        .unwrap_or_else(|error| {
            panic!("Could not read file: {:?}", error);
        })
        .split("\n\n")
        .map(|m| {
            let mut l = m.lines();
            let mut mon = Monkey::new();
            mon.index = monkey_index
                .captures(l.next().unwrap())
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            mon.items = l
                .next()
                .unwrap()
                .split(": ")
                .last()
                .unwrap()
                .split(", ")
                .map(|item| item.parse::<u64>().unwrap())
                .collect();
            let oplist: Vec<&str> = l.next().unwrap().split(' ').collect();
            match oplist[6] {
                "*" => {
                    if oplist[7] == "old" { mon.pow = 2;}
                    else { mon.mult = oplist[7].parse::<u32>().unwrap_or(1)}
                },
                "+" => {
                    if oplist[7] == "old" { mon.mult = 2;}
                    else { mon.add = oplist[7].parse::<u32>().unwrap_or(1)}
                },
                _ => println!("Could not parse Operation, got {:?}", oplist),
            };
            mon.div = l
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            mon.test1 = l
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            mon.test0 = l
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            mon
        })
        .collect();

    println!("{}", run_inspection(monkeys.clone(), 'a'));
    println!("{}", run_inspection(monkeys.clone(), 'b'));

}

#[derive(Debug, Clone)]
struct Monkey {
    index: usize,
    items: VecDeque<u64>,
    add: u32,
    mult: u32,
    pow: u32,
    div: u32,
    test1: usize,
    test0: usize,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            index: 0,
            items: VecDeque::new(),
            add: 0,
            mult: 1,
            pow: 1,
            div: 1,
            test1: 0,
            test0: 0,
        }
    }
}

fn run_inspection(mut monkeys: Vec<Monkey>, part: char) -> u64 {
    let mut inspections = [0u64; 10];
    let rounds = match part {
        'b' => 10000,
        _ => 20,
    };
    let big_relief = monkeys
        .iter()
        .map(|m| m.div as u64)
        .product::<u64>();
    for _r in 0..rounds {
        for m in 0..monkeys.len() {
            let num_items = monkeys[m].items.len();
            inspections[m] += num_items as u64;
            for _i in 0..num_items {
                // Inspect
                let mut item = monkeys[m].items.pop_front().unwrap();
                item = item + monkeys[m].add as u64;
                item = item * monkeys[m].mult as u64;
                item = item.pow(monkeys[m].pow);   
                // Relief
                match part {
                    'b' => item = item % big_relief,
                    _ => item = item/3,
                };
                // Test
                if item % monkeys[m].div as u64 == 0 {
                    let target = monkeys[m].test1;
                    monkeys[target].items.push_back(item);
                } else {
                    let target = monkeys[m].test0;
                    monkeys[target].items.push_back(item);
                }
            }
        }
    }
    inspections
        .into_iter()
        .sorted()
        .rev()
        .take(2)
        .collect::<Vec<u64>>()
        .into_iter()
        .product::<u64>()
}