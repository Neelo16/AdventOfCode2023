use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error parsing line"))
        .collect();
    let times: Vec<u64> = lines
        .first().unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let records: Vec<u64> = lines.last().unwrap().split_whitespace().skip(1).map(|s| s.parse::<u64>().unwrap()).collect();

    let mut result = 1;
    for (time, record) in zip(times, records) {
        let mut count = 0;
        for t in 0..time {
            let d = distance_covered(t, time);
            if d > record {
                count += 1;
            }
        }
        result *= count;
    }
    println!("First star: {}", result);
    let mut count = 0;
    let time = lines
        .first().unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>().unwrap();
    let record = lines
        .last().unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>().unwrap();
    for t in 0..time {
        let d = distance_covered(t, time);
        if d > record {
            count += 1;
        }
    }
    println!("Second star: {}", count);
}

fn distance_covered(hold_time: u64, race_time: u64) -> u64 {
    return (race_time - hold_time) * hold_time;
}