use std::env;
use std::fs;
use advent2021::find_filename;

fn main() {

    let filename = find_filename(env::args());

    let scores = [4,8,3,1,5,9,7,2,6];
    let total: Vec<u32> = fs::read_to_string(filename)
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
        }).map(|index: usize| scores[index]).collect::<Vec<u32>>();

    println! ("{}", total.iter().sum::<u32>());
}
