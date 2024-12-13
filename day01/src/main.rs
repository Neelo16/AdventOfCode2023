use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn calibrate(line: &String, translate: bool) -> i32 {
    let translator = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "4"),
        ("five", "5e"),
        ("six", "6"),
        ("seven", "7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ]);

    let mut translated_line: String = String::from(line);

    if translate {
        for (key, translation) in translator {
            translated_line = translated_line.replace(&key, translation);
        }
    }

    let first_digit = translated_line.chars().find(|c| c.is_digit(10)).unwrap().to_string();
    let last_digit = translated_line.chars().rfind(|c| c.is_digit(10)).unwrap().to_string();

    return [first_digit, last_digit].join("").parse().expect("Invalid line does not match pattern");
}

fn main() {
    let file = File::open("input.txt").expect("No such file");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    println!("First star: {}", lines.iter().map(|l| calibrate(l, false)).sum::<i32>());
    println!("Second star: {}", lines.iter().map(|l| calibrate(l, true)).sum::<i32>());
}
