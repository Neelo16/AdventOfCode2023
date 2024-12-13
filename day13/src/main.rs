use std::fs::File;
use std::io::{BufRead, BufReader};

type Grid = Vec<String>;

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error parsing line"))
        .collect();

    let grids: Vec<Grid> = parse_input(lines);
    println!("First star: {}", grids.iter().map(|g| solve(g, false)).sum::<usize>());
    println!("Second star: {}", grids.iter().map(|g| solve(g, true)).sum::<usize>());
}

fn solve(grid: &Grid, smudge: bool) -> usize {
    for i in 1..grid.first().unwrap().len() {
        if check_reflection(grid, i, smudge) {
            return i;
        }
    }
    let transposed = transpose(grid);
    for i in 1..transposed.first().unwrap().len() {
        if check_reflection(&transposed, i, smudge) {
            return 100 * i;
        }
    }
    panic!("No reflections found");
}


fn check_reflection(grid: &Grid, start: usize, smudge: bool) -> bool {
    let mut smudge_found = false;
    for row in grid {
        let left = row[0..start].chars().rev();
        let right = &row[start..row.len()];
        let differences = right.chars().zip(left).filter(|(a, b)| a != b).count();
        if smudge && !smudge_found && differences == 1 {
            smudge_found = true;
            continue;
        }
        if differences != 0 {
            return false;
        }
    }
    return !smudge || smudge_found;
}

fn transpose(grid: &Grid) -> Grid {
    let mut transposed = Vec::with_capacity(grid.first().unwrap().capacity());
    let chars: Vec<Vec<char>> = grid.iter().map(|s| s.chars().collect()).collect();
    for _ in 0..grid.first().unwrap().len() {
        transposed.push(Vec::with_capacity(grid.capacity()));
    }

    for i in 0..grid.first().unwrap().len() {
        for j in 0..grid.len() {
            transposed[i].push(chars[j][i]);
        }
    }

    return transposed.iter().map(|s| String::from_iter(s.iter())).collect();
}

fn parse_input(lines: Vec<String>) -> Vec<Grid> {
    let mut grids = vec![];
    let mut current_grid = vec![];
    for line in &lines {
        if line.is_empty() {
            grids.push(current_grid);
            current_grid = vec![];
        } else {
            current_grid.push(line.chars().collect());
        }
    }
    grids.push(current_grid);
    return grids;
}

fn _display(grid: &Grid) {
    for line in grid {
        println!("{line}")
    }
}