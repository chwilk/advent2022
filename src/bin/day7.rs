use std::env;
use std::fs;
use advent2022::find_filename;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {

    let filename = find_filename(env::args());

    let input = fs::read_to_string(filename)
        .unwrap_or_else(|error| {
            panic!("Could not read file: {:?}", error);
        });
    let commands=input.lines();

    let mut pwd = String::from("/");
    let mut dirs = HashMap::new();

    for line in commands {
        let args: Vec<&str> = line.split(' ').collect();
        if args[0] == "$" {
            match args[1] {
                 "cd" => {
                    cd(&mut pwd, args[2].to_string());
                 },
                 &_ => ()
            }
        } else {
            let s = args[0].parse::<u64>().unwrap_or(0);
            if s > 0 {
                let val = s + dirs.get(&pwd).unwrap_or(&0);
                dirs.insert(pwd.clone(), val);
            } else {
                let path = if pwd == "/" {
                    format!("/{}", args[1])
                } else {
                    format!("{}/{}", pwd.clone(), args[1])
                };
                dirs.insert(path, 0);
            }
        }
    }
    let mut sumdirs = dirs.clone();
    let paths: Vec<&String> = dirs.keys().sorted().rev().collect();
    for &d in paths.iter() {
        if d == "/" { break; }
        let mut dir = d.clone();
        let mut dirsize = *sumdirs.get(&dir).unwrap_or(&0);
        dir = dir.rsplit_once('/').unwrap().0.to_string();
        if dir.len() == 0 {
            dir = "/".to_string();
        }
        dirsize = dirsize + *sumdirs.get(&dir).unwrap_or(&0);
        sumdirs.insert(dir.clone(), dirsize);
    }
    println!("{:?}", sumdirs
        .values()
        .filter(|s| *s<&100000) 
        .sum::<u64>());

    let target = 30000000 - (70000000 - sumdirs.get("/").unwrap());
    println!("{:?}", sumdirs
        .values()
        .filter(|s| *s>=&target) 
        .sorted()
        .rev()
        .last()
        .unwrap());
}

fn cd(pwd: &mut String, path: String)  {
    if path.chars().nth(0) == Some('/') {
        *pwd = path;
    } else if path == ".." {
        *pwd = pwd.rsplit_once('/').unwrap().0.to_string();
        if pwd.len() == 0 {
            *pwd = "/".to_string()
        }
    } else if pwd.ends_with('/') {
        *pwd = format!("{}{}", pwd, path)
    } else {
        *pwd = format!("{}/{}", pwd, path)
    }
    ()
}