use std::env;
use std::fs;
use std::collections::HashMap;
use enum_as_inner::EnumAsInner;
use advent2022::find_filename;

#[derive(Debug, Clone, EnumAsInner, Eq, Hash, PartialEq)]
enum Token {
    NUM(i64),
    MONKEY(String),
    OP(char),
}
fn main() {
    use crate::Token::*;
    let filename = find_filename(env::args());
    let mut puzzle = HashMap::new();

    let rules = fs::read_to_string(filename)
        .unwrap_or_else(|error| {
            panic!("Could not read file: {:?}", error);
        })
        .lines()
        .map(|i| {
            i.split(|c| (c == ' ') || (c == ':'))
                .filter_map(|tok| {
                    if let Some(first) = tok.chars().nth(0) {
                        match first {
                            'a'..='z' => Some(MONKEY(tok.to_string())),
                            '0'..='9' => Some(NUM(tok.parse::<i64>().unwrap())),
                            _         => Some(OP(first)),
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<Token>>()
        })
        .collect::<Vec<Vec<Token>>>();

    for rule in rules {
        let monkey = rule[0].clone();
        let commands = rule[1..rule.len()].to_vec();
        //println!("{:?} := {:?}", monkey, commands);
        puzzle.insert(monkey, commands);
    }

    while puzzle.get(&MONKEY("root".to_string())).unwrap().len() > 1 {
        for monkey in puzzle.clone().keys() {
            let expr = puzzle.get(monkey).unwrap();
            if expr.len() == 3 {
                let lhv = puzzle.get(&expr[0]).unwrap();
                let op = &expr[1];
                let rhv = puzzle.get(&expr[2]).unwrap();
                if lhv.len() == 1 && rhv.len() == 1 {
                    let mky = monkey.clone();
                    let lhs = lhv[0].as_num().unwrap();
                    let rhs = rhv[0].as_num().unwrap();
                    let res = match op.as_op().unwrap() {
                        '+' => lhs + rhs,
                        '*' => lhs * rhs,
                        '-' => lhs - rhs,
                        '/' => lhs / rhs,
                        _   => panic!("Illegal operand character \"{:?}\"", op),
                    };
                    puzzle.insert(mky, [NUM(res)].to_vec());
                }
            } 
        }
    }
    println!("{}", puzzle.get(&MONKEY("root".to_string()))
        .unwrap()[0]
        .as_num()
        .unwrap());

}
