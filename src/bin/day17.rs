use std::env;
use std::fs;
use advent2022::find_filename;

#[derive(Debug, Clone, Copy )]
enum Move {
    L,
    R,
    D,
}

#[derive(Debug, Clone, Copy )]
enum Rock {
    Horiz  = 0x0000003c,
    Plus   = 0x00103810,
    Ell    = 0x00080838,
    Vert   = 0x20202020,
    Square = 0x00003030,
}

fn main() {
    use crate::Move::{L,R};
    let filename = find_filename(env::args());

    let jets: Vec<Move> = fs::read_to_string(filename)
        .unwrap_or_else(|error| {
            panic!("Could not read file: {:?}", error);
        })
        .chars()
        .filter_map(|m| match m {
            '<' => Some(L),
            '>'  => Some(R),
            _ => None,
        })
        .collect();
    println!("{}", play(jets.clone(), 2022));
}

fn play (jets: Vec<Move>, turns: usize) -> usize {
    use crate::Move::{L,R,D};
    use crate::Rock::{*};

    let mut chamber = Vec::new();
    chamber.push(0xff as u8);

    let mut jet: usize = 0;

    for turn in 0..turns {
        let rock = match turn % 5 {
            0 => Horiz,
            1 => Plus,
            2 => Ell,
            3 => Vert,
            _ => Square,
        };
        let mut shift = 0;
        let mut insert = chamber.len()+3;
        for _j in 0..7 { // load blank space above current sculpture
            chamber.push(0);
        }
        loop {
            match try_move(&rock, shift, jets[jet], &chamber[insert-1..insert+4]) {
                true => {
                    shift += match jets[jet] {
                        L => -1,
                        R => 1,
                        D => 0,
                    }
                },
                false => (),
            }
            jet = (jet + 1)%jets.len();
            match try_move(&rock, shift, D, &chamber[insert-1..insert+4]) {
                true => { insert -= 1; },
                false => break,
            }
        }
        // Place piece
        let print_rock = place_rock(&rock, shift);
        for i in 0..4 {
            chamber[insert+i] = chamber[insert+i] | print_rock[i];
        }
        // Clear blank lines from top
        while chamber.last().unwrap() == &0 { chamber.pop();}
        //print_map(chamber.clone());
    }
    chamber.len() - 1
}

fn try_move (rock: &Rock, shift: i32, mov: Move, chamber: &[u8]) -> bool {
    use crate::Move::{L,R,D};
    let mut r = place_rock(&rock, shift);
    let mut offset:usize = 1;
    match mov {
        L => {
            if shift == -2 { return false; }
            r = place_rock(&rock, shift - 1);
        },
        R => {
            for i in 0..4 {
                if r[i] & 0x03 != 0 { return false;}
            }
            r = place_rock(&rock, shift + 1);
        },
        D => {
            offset = 0;
        },
    }
    for i in 0..4 {
        if r[i] & chamber[i+offset] != 0 { return false; }
    }
    true
}

fn place_rock (rock: &Rock, shift: i32) -> [u8;4] {
    let mut r = (*rock as u32).to_le_bytes();
    match shift {
        -2 | -1 => 
            for i in 0..4 {
                r[i] = r[i] << (-shift);
            },
        0 => (),
        1..=4 =>
            for i in 0..4 {
                r[i] = r[i] >> shift;
            },
        _ => panic!("Illegal shift in place_rock: {}", shift),
    }
    r
}

#[allow(dead_code)]
fn print_map(map: Vec<u8>) -> () {
    for row in (0..map.len()).rev() {
        let line = format!("{:08b}", map[row]).
            chars()
            .map(|c| match c {
                '0' => ".",
                _   => "#",
            })
            .collect::<Vec<&str>>()
            .concat();
        println!("{}", line);
    }
    print!("\n");
}