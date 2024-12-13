use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error parsing line"))
        .collect();
    let sequences: Vec<Vec<i64>> = lines.iter()
        .map(|s| s.split_whitespace().map(|i| i.parse::<i64>().unwrap()).collect())
        .collect();
    println!("First star: {}", sequences.iter().map(next_value).sum::<i64>());

    let reverted_sequences: Vec<_> = sequences.iter().map(|s| s.iter().cloned().rev().collect()).collect();
    println!("Second star: {}", reverted_sequences.iter().map(next_value).sum::<i64>());
}

fn next_value(seq: &Vec<i64>) -> i64 {
    let mut sequences = vec![seq.clone()];
    while !sequences.last().unwrap().iter().all(|i| *i == 0) {
        let last_sequence = sequences.last().unwrap();
        let mut next_sequence = Vec::with_capacity(last_sequence.len() - 1);
        for (v1, v2) in zip(&last_sequence[0..last_sequence.len()-1], &last_sequence[1..]) {
            next_sequence.push(v2 - v1);
        }
        sequences.push(next_sequence);
    }
    sequences.last_mut().unwrap().push(0);
    for i in (0..sequences.len()-1).rev() {
        let next_sequence = sequences[i+1].clone();
        let sequence = &mut sequences[i];
        sequence.push(sequence.last().unwrap() + next_sequence.last().unwrap());
    }
    return *sequences.first().unwrap().last().unwrap();
}
