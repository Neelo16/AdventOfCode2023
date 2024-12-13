use std::collections::VecDeque;
use std::fs::File;
use memoize::memoize;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error parsing line"))
        .collect();
    let rows: Vec<(String, VecDeque<i64>)> = lines
        .iter()
        .map(parse_line)
        .collect();
    let unfolded_rows: Vec<_> = rows
        .iter()
        .map(|(line, counts)| unfold(line, counts))
        .collect();
    println!("First star: {}", rows.iter().map(|(line, counts)| count_arrangements(line.clone(), counts.clone())).sum::<i64>());
    println!("Second star: {}", unfolded_rows.iter().map(|(line, counts)| count_arrangements(line.clone(), counts.clone())).sum::<i64>());
}

fn unfold(line: &String, counts: &VecDeque<i64>) -> (String, VecDeque<i64>) {
    let mut unfolded_counts = VecDeque::new();
    for _ in 0..5 {
        for i in counts {
            unfolded_counts.push_back(*i);
        }
    }
    return (vec![line].repeat(5).iter().map(|s| s.to_string()).collect::<Vec<String>>().join("?"), unfolded_counts);
}

#[memoize]
fn count_arrangements(line: String, counts: VecDeque<i64>) -> i64 {
    if counts.is_empty() {
        if line.chars().filter(|c| *c == '#').count() == 0 {
            return 1;
        }
        return 0;
    }
    if line.is_empty() {
        return 0;
    }
    if line.starts_with("#") {
        let mut new_counts = counts.clone();
        let mut chars_to_cut = new_counts.pop_front().unwrap() as usize + 1;
        if line.len() < chars_to_cut - 1 || line[0..chars_to_cut - 1].chars().any(|c| c == '.') {
            return 0;
        }
        if line.len() >= chars_to_cut {
            if line[chars_to_cut-1..chars_to_cut].starts_with('#') {
                return 0;
            }
        } else {
            chars_to_cut -= 1;
        }
        return count_arrangements(line[chars_to_cut..line.len()].to_string(), new_counts);
    }
    let tail = line[1..line.len()].to_string();
    if line.starts_with(".") {
        return count_arrangements(tail, counts);
    }
    return count_arrangements(format!("#{tail}"), counts.clone()) + count_arrangements(format!("{tail}"), counts);
}

fn parse_line(line: &String) -> (String, VecDeque<i64>) {
    let mut split = line.split_whitespace();
    let springs = split.next().unwrap().to_string();
    let counts = split.next().unwrap().split(",").map(|s| s.parse::<i64>().unwrap()).collect();
    return (springs, counts);
}
