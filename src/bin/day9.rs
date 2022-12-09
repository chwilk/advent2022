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

    let mut loc = HashSet::new();
    let mut head = Point{ x:0, y:0};
    let mut tail = Point{ x:0, y:0};

    loc.insert(tail);

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
            head = head + mov;
            tail.follow(head);
            loc.insert(tail);
            //println! ("H{:?}, T{:?}", head, tail );
        }

    }
    println!("{}", loc.len());
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
            self.x = other.x;
        } else if self.y - other.y > 1 {
            self.y = self.y - 1;
            self.x = other.x;
        }
        if other.x - self.x > 1 {
            self.x = self.x + 1;
            self.y = other.y;
        } else if self.x - other.x > 1 {
            self.x = self.x - 1;
            self.y = other.y;
        }
        self
    }
}