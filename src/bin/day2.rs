use std::env;
use std::fs;
use advent2021::find_filename;

struct Day {
    a: u32,
    b: u32,
}

fn main() {

    let filename = find_filename(env::args());

    let parta = [4,8,3,1,5,9,7,2,6];
    let partb = [3,4,8,1,5,9,2,6,7];
    let total: Vec<Day> = fs::read_to_string(filename)
        .unwrap_or_else(|error| {
            panic!("Could not read file: {:?}", error);
        })
        .lines()
        .map(|round| {
            round.split(' ')
                .map(|code|
                    match code {
                        "X" => 0,
                        "Y" => 1,
                        "Z" => 2,
                        "B" => 3,
                        "C" => 6,
                        &_  => 0,
                    }).collect::<Vec<usize>>().iter().sum()
        }).map(|index: usize| Day{a: parta[index], b: partb[index]}).collect::<Vec<Day>>();

    println! ("{}", total.iter().map(|day| day.a).sum::<u32>());
    println! ("{}", total.iter().map(|day| day.b).sum::<u32>());
}
