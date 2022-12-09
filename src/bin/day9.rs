use std::env;
use std::fs;
use std::ops::Add;
use advent2022::find_filename;
use std::collections::HashSet;

fn main() {

    let filename = find_filename(env::args());

    let moves  = fs::read_to_string(filename)
        .unwrap_or_else(|error| {
            panic!("Could not read file: {:?}", error);
        });

    let mut parta = HashSet::new();
    let mut partb = HashSet::new();
    let mut rope = [Point{ x:0, y:0};10];

    parta.insert(rope[1]);
    partb.insert(rope[9]);

    for cmd in moves.lines() {
        let (dir, len) = cmd.split_once(' ').unwrap();
        let mov = match dir {
            "R" => Point {x:1, y:0},
            "L" => Point {x:-1, y:0},
            "U" => Point {x:0, y:-1},
            "D" => Point {x:0, y:1},
            _ => Point {x:0,y:0},
        };
        for _i in 0..len.parse::<usize>().unwrap_or(0) {
            rope[0] = rope[0] + mov;
            for i in 1..10 {
                rope[i].follow(rope[i-1]);
            }
            parta.insert(rope[1]);
            partb.insert(rope[9]);
            //println! ("H{:?}, T{:?}", head, tail );
        }

    }
    println!("{}", parta.len());
    println!("{}", partb.len());
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    fn follow(&mut self, other: Point) -> &mut Point {
        if other.y - self.y > 1 {
            self.y = self.y + 1;
            if self.x > other.x {
                self.x = self.x - 1;
            } else if self.x < other.x {
                self.x = self.x + 1;
            }
        } else if self.y - other.y > 1 {
            self.y = self.y - 1;
            if self.x > other.x {
                self.x = self.x - 1;
            } else if self.x < other.x {
                self.x = self.x + 1;
            }
        } else if other.x - self.x > 1 {
            self.x = self.x + 1;
            if self.y > other.y {
                self.y = self.y - 1;
            } else if self.y < other.y {
                self.y = self.y + 1;
            }
        } else if self.x - other.x > 1 {
            self.x = self.x - 1;
            if self.y > other.y {
                self.y = self.y - 1;
            } else if self.y < other.y {
                self.y = self.y + 1;
            }
        }
        self
    }
}