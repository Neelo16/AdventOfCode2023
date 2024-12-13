use std::fs::File;
use std::io::{BufRead, BufReader};

struct Lens {
    label: String,
    focal_length: u8
}

enum Operation {
    ADD,
    REMOVE
}

fn main() {
    let file = File::open("bigboy.txt").expect("File not found");
    let line: String = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error parsing line"))
        .next()
        .unwrap();
    println!("First star: {}", line.split(",").map(hash).sum::<i64>());

    let mut hash_table: Vec<Vec<Lens>> = Vec::with_capacity(256);
    for _ in 0..256 {
        hash_table.push(vec![]);
    }

    for operation in line.split(",") {
        let (operation, lens) = parse(operation);
        let lens_box = &mut hash_table[hash(lens.label.as_str()) as usize];
        match operation {
            Operation::ADD => {
                match lens_box.iter().position(|l| l.label == lens.label) {
                    None => { lens_box.push(lens) }
                    Some(i) => { lens_box[i] = lens }
                }
            },
            Operation::REMOVE => {
                match lens_box.iter().position(|l| l.label == lens.label) {
                    None => {}
                    Some(i) => { lens_box.remove(i); }
                }
            }
        }
    }

    let mut focusing_power: usize = 0;

    for (i, lens_box) in hash_table.iter().enumerate() {
        for (j, lens) in lens_box.iter().enumerate() {
            focusing_power += (i + 1) * (j + 1) * lens.focal_length as usize;
        }
    }

    println!("Second star: {focusing_power}");
}

fn hash(s: &str) -> i64 {
    return s.chars().fold(0, |acc, c| (acc + (c as i64)) * 17 % 256);
}

fn parse(s: &str) -> (Operation, Lens) {
    return if s.contains("-") {
        (Operation::REMOVE, Lens { label: s.strip_suffix("-").unwrap().to_string(), focal_length: 0 })
    } else if s.contains("=") {
        let mut split = s.split("=");
        let label = split.next().unwrap().to_string();
        let focal_length = split.next().unwrap().parse::<u8>().unwrap();
        (Operation::ADD, Lens { label, focal_length })
    } else {
        panic!("Invalid operation");
    }
}