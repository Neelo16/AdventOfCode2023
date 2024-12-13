use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = (usize, usize);

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error parsing line"))
        .collect();
    let mut galaxies: Vec<Coord> = vec![];
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let (empty_lines, empty_columns) = expand(&grid);

    for (y, line) in grid.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == '#' {
                galaxies.push((x, y))
            }
        }
    }

    let mut result = 0;
    let mut result2 = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for galaxy2 in galaxies[i+1..galaxies.len()].iter() {
            let length = galaxy.0.abs_diff(galaxy2.0) + galaxy.1.abs_diff(galaxy2.1);
            let empty_spaces = empty_lines.iter()
                .filter(|y| y > min(&&galaxy.1, &&galaxy2.1) && y < max(&&galaxy.1, &&galaxy2.1))
                .count()
                +
                empty_columns.iter()
                    .filter(|x| x > min(&&galaxy.0, &&galaxy2.0) && x < max(&&galaxy.0, &&galaxy2.0))
                    .count();
            result += length + empty_spaces;
            result2 += length + (empty_spaces * 999999);
        }
    }
    println!("First star: {}", result);
    println!("Second star: {}", result2);
}

fn expand(grid: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut empty_lines = vec![];
    let mut empty_columns = vec![];

    for (y, line) in grid.iter().enumerate() {
        if line.iter().all(|c| *c == '.') {
            empty_lines.push(y);
        }
    }

    for x in 0..grid.first().unwrap().len() {
        let mut empty = true;
        for y in 0..grid.len() {
            if grid[y][x] != '.' {
                empty = false;
                break;
            }
        }
        if empty {
            empty_columns.push(x);
        }
    }

    return (empty_lines, empty_columns);
}
