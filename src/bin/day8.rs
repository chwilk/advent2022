use std::env;
use std::fs;
use advent2022::find_filename;

fn main() {

    let filename = find_filename(env::args());

    let mut width = 0;

    let forest: Vec<u8> = fs::read_to_string(filename)
        .unwrap_or_else(|error| {
            panic!("Could not read file: {:?}", error);
        })
        .lines()
        .map(|l| {
            if width == 0 {
                width = l.len();
            }
            l.chars().map(|c| (c as u8) - 48 ).collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
        .concat();
    let length = forest.len() / width;
    let tforest = transpose(forest.clone(), width, length);

    let mut visible_a = (length + width - 2)*2;
    let mut vistas: Vec<u32> = vec![0; length*width];

    for i in 1..(length-1) {
        for j in 1..(width-1) {
            let idx = j+i*width;
            let tidx = i+j*length;
            if is_visible(forest[idx], forest[(i*width)..idx].to_vec())
                || is_visible(forest[idx], forest[idx+1..((i+1)*width)].to_vec())
                || is_visible(forest[idx], tforest[j*length..tidx].to_vec())
                || is_visible(forest[idx], tforest[tidx+1..((j+1)*length)].to_vec())
            {
                visible_a += 1;
            }
            vistas[idx] = count_view(forest[idx], forest[(i*width)..idx].to_vec(), true)
                * count_view(forest[idx], forest[idx+1..((i+1)*width)].to_vec(), false)
                * count_view(forest[idx], tforest[j*length..tidx].to_vec(), true)
                * count_view(forest[idx], tforest[tidx+1..((j+1)*length)].to_vec(), false)
        }
    }

    vistas.sort();

    println!("{}", visible_a);
    println!("{:?}", vistas.last().unwrap());
    
}

fn is_visible(tree: u8, others: Vec<u8>) -> bool {
    for o in others {
        if o >= tree {
            return false;
        }
    }
    true
}

fn count_view(tree: u8, mut others: Vec<u8>, reverse: bool) -> u32 {
    let mut sum = 0;
    if reverse {
        others.reverse();
    }
    for o in others {
        if o < tree {
            sum += 1;
        } else {
            sum += 1;
            break;
        }
    }
    sum
}

fn transpose(forest: Vec<u8>, width: usize, length: usize) -> Vec<u8> {
    let mut res = Vec::new();
    for i in 0..length {
        for j in 0..width {
            res.push(forest[i+j*width])
        }
    }
    res
}