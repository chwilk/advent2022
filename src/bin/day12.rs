use std::env;
use itertools::Itertools;
use std::fs;
use rgb2ansi256::rgb_to_ansi256;
use advent2022::find_filename;

fn main() {

    let filename = find_filename(env::args());

    let mut width = 0;
    let mut start = 0;
    let mut end = 0;
    let mut row = 0;

    let height_map: Vec<char> = fs::read_to_string(filename)
        .unwrap_or_else(|error| {
            panic!("Could not read file: {:?}", error);
        })
        .lines()
        .map(|l| {
            row += 1;
            if width == 0 {
                width = l.len();
            }
            l.chars()
            .enumerate()
            .map(|(i, c)| match c {
                'S' => {start = i+(row-1)*width; 'a'}
                'E' => {end = i+(row-1)*width; 'z'}
                _ => c
            })
            .collect::<Vec<char>>()
        })
        .concat();

        println!("{}", parta(height_map.clone(), width, start, end));
}

fn parta(map: Vec<char>, width: usize, start: usize, end: usize) -> usize {
    let infinity = width * width * width;
    let mut dist: Vec<usize> = Vec::new();
    let mut unvisited = Vec::new();

    for v in 0..map.len() {
        dist.push(infinity);
        unvisited.push(v);
    }
    dist[start] = 0;
    
    loop {
        if unvisited.len() == 0 { break;}
        let mut u = 0;
        let mut min = infinity;
        let mut idx = 0;
        for (u_i, u_u) in unvisited.clone().into_iter().enumerate() {
            if min > dist[u_u] {
                u = u_u;
                idx = u_i;
                min = dist[u_u];
            }  
        }
        if u == end {
            break;
        }
        unvisited.remove(idx);
        // check neighbors
        let mut neigh = Vec::new();
        if u % width > 0 { //neighbor to the left?
            neigh.push(u-1);
        }
        if u % width < width-1 {// neighbor to the right?
            neigh.push(u+1);
        }
        if u >= width {// neighbor to top
            neigh.push(u-width);
        }
        if u < map.len() - width {// neighbor to the bottom?
            neigh.push(u+width);
        }
        for n in neigh {
            if unvisited.contains(&n) {
                let jump = map[n] as i32 - map[u] as i32;
                if jump < 2 {
                    if dist[u] + 1 < dist[n] {
                        dist[n] = dist[u] + 1;
                    }
                }
            }
        }
    }
    
    dist[end]
}

#[allow(dead_code)]
fn print_map(map: Vec<char>, width: usize) -> () {
    for (i, c) in map.iter().enumerate() {
        let color = match c {
            'a'..='z' => rgb_to_ansi256((*c as u8 - 97)*9, (*c as u8 - 97)*9, 20 ),
            _ => rgb_to_ansi256(255, 255, 0 ),
        };
        print!("\x1b[38;5;{}m{}", color, c);
        if i%width == width-1 { print!("\x1b[0m\n");}
    }
}