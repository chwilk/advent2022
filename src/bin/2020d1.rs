use std::env;
use itertools::Itertools;
use std::fs;
use advent2021::find_filename;

fn main() {

    let filename = find_filename(env::args());

    let entries: Vec<usize> = fs::read_to_string(filename)
        .unwrap_or_else(|error| {
            panic!("Could not read file: {:?}", error);
        })
        .lines()
            .map(|entry| entry.parse::<usize>().unwrap_or(0))
        .sorted()
        .collect();
    for i in 0..(entries.len()-1) {
        match entries.split_at(i).1.binary_search(&(2020-entries[i])) {
                Ok(idx) => {
                    println! ("{}*{} = {}", entries[i], entries[idx+i], entries[i] * entries[idx+i]);
                },
                _ => (),
        }
        for j in i..(entries.len()-2) {
            if entries[i] + entries[j] < 2020 {
                match entries.split_at(j).1.binary_search(&(2020-entries[i]-entries[j])) {
                    Ok(idx) => {
                        println! ("{}*{}*{} = {}", entries[i], entries[j], entries[idx+j],
                            entries[i]*entries[j]*entries[idx+j])
                    },
                    _ => (),
                }
            }
        }
    }
}
