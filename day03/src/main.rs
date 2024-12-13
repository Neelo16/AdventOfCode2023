use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const EMPTY: i64 = -1;
const GEAR: i64 = -2;
const OTHER: i64 = -3;

fn gear_ratio(s: &Vec<i64>) -> i64 {
    return s.iter().map(|i| *i).reduce(|acc, i| acc * i).unwrap();
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error parsing line"))
        .collect();

    let mut matrix: Vec<Vec<i64>> = Vec::with_capacity(lines.len());

    for line in &lines {
        matrix.push(Vec::with_capacity(line.len()));
    }


    for (y, line) in lines.iter().enumerate() {
        for char in line.chars() {
            let value: i64 = if char.is_digit(10) {
                char.to_digit(10).unwrap() as i64
            } else if char == '.' {
                EMPTY
            } else if char == '*' {
                GEAR
            } else {
                OTHER
            };
            matrix[y].push(value);
        }
    }


    let mut sum = 0;
    let mut gear_adjacent_parts: HashMap<(i64, i64), Vec<i64>> = HashMap::new();

    let max_y = matrix.len() as i64;
    let max_x = matrix.first().unwrap().len() as i64;

    for (y_base, row) in matrix.iter().enumerate() {
        let mut current_number = 0;
        let mut is_number_valid = false;
        let mut checked_positions: HashSet<(i64, i64)> = HashSet::new();

        for (x_base, char) in row.iter().enumerate() {
            let x: i64 = x_base as i64;
            let y: i64 = y_base as i64;


            let positions_to_check: &Vec<(i64, i64)> = &[(x, y - 1), (x, y + 1),
                (x + 1, y), (x - 1, y),
                (x + 1, y + 1), (x + 1, y - 1),
                (x - 1, y + 1), (x - 1, y - 1)
            ]
                .iter()
                .filter(|(x2, y2)| 0 <= *x2 && x2 < &max_x && 0 <= *y2 && y2 < &max_y)
                .map(|c| *c)
                .collect();
            if *char < 0 {
                if is_number_valid && current_number > 0 {
                    sum += current_number;
                    for (x2, y2) in &checked_positions {
                        if matrix[*y2 as usize][*x2 as usize] == GEAR {
                            let count = gear_adjacent_parts.get_mut(&(*x2, *y2));
                            match count {
                                Some(vec) => {
                                    vec.push(current_number);
                                }
                                None => {
                                    gear_adjacent_parts.insert((*x2, *y2), vec![current_number]);
                                }
                            }
                        }
                    }
                }
                current_number = 0;
                is_number_valid = false;
                checked_positions.clear();
            } else {
                current_number = current_number * 10 + char;
                is_number_valid = is_number_valid || positions_to_check
                    .iter()
                    .any(|(x2, y2)| [OTHER, GEAR].contains(&matrix[*y2 as usize][*x2 as usize]));
                for position in positions_to_check {
                    checked_positions.insert(*position);
                }
            }
        }
        if is_number_valid && current_number > 0 {
            sum += current_number;
            for (x2, y2) in &checked_positions {
                if matrix[*y2 as usize][*x2 as usize] == GEAR {
                    let count = gear_adjacent_parts.get_mut(&(*x2, *y2));
                    match count {
                        Some(vec) => {
                            vec.push(current_number);
                        }
                        None => {
                            gear_adjacent_parts.insert((*x2, *y2), vec![current_number]);
                        }
                    }
                }
            }
        }
    }

    println!("First star: {}", sum);
    println!("Second star: {}", gear_adjacent_parts.values().filter(|v| v.len() == 2).map(gear_ratio).sum::<i64>());
}
