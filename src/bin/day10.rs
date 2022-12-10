use std::env;
use std::fs;
use advent2022::find_filename;

fn main() {

    let filename = find_filename(env::args());

    let program = fs::read_to_string(filename)
        .unwrap_or_else(|error| {
            panic!("Could not read file: {:?}", error);
        });
    let mut register:i32 = 1;
    let mut cycle: i32 = 1;
    let mut sum = 0;
    let mut prior = -22;

    for instruction in program.lines() {
        let args = instruction.split(' ').collect::<Vec<&str>>();
        let modulus = (cycle as i32 + 21) % 40;
        if modulus < prior {
            let key_cycle = ((cycle-18)/40)*40 + 20;
            sum += key_cycle*register;
        }
        prior = modulus;
        match args[0] {
            "noop" => {
                print!("{}", match (cycle-1)%40-register {
                    -1..=1 => '#',
                    _ => '.',
                });
                if cycle%40 == 0 {
                    print!("\n");
                }
                cycle += 1;
            },
            "addx" => {
                for _i in 0..2 {
                    print!("{}", match (cycle-1)%40-register {
                        -1..=1 => '#',
                        _ => '.',
                    });
                    if cycle%40 == 0 {
                        print!("\n");
                    }
                    cycle += 1;
                }
                register += args[1].parse::<i32>().unwrap_or(0);
            },
            _ => ()
        }
    }
    println!("{}", sum);
}
