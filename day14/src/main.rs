use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Grid = Vec<Vec<char>>;

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let mut lines: Grid = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error parsing line").chars().collect())
        .collect();

    let (width, height) = (lines.first().unwrap().len(), lines.len());

    let directions: [(Vec<usize>, Vec<usize>, bool, fn(&mut Grid, usize, usize, usize), fn(usize, usize) -> i64); 4] = [
        ((0..width).collect(), (0..height).collect(), false, |g, x, y, top| {
            g[y][x] = '.';
            g[top][x] = 'O';
        }, |_, y| y as i64 + 1),
        ((0..width).collect(), (0..height).collect(), true, |g, x, y, top| {
            g[y][x] = '.';
            g[y][top] = 'O';
        }, |x, _| x as i64 + 1),
        ((0..width).collect(), (0..height).rev().collect(), false, |g, x, y, top| {
            g[y][x] = '.';
            g[top][x] = 'O';
        }, |_, y| y as i64 - 1),
        ((0..width).rev().collect(), (0..height).collect(), true, |g, x, y, top| {
            g[y][x] = '.';
            g[y][top] = 'O';
        }, |x, _| x as i64 - 1),
    ];

    let mut grid = lines.clone();
    let (x_values, y_values, horizontal, swap, next_top) = directions.first().unwrap();
    simulate(x_values, y_values, &mut grid, *horizontal, *swap, *next_top);

    let load = total_load(&grid, width, height);
    println!("First star: {}", load);

    let mut configurations = HashSet::new();

    let mut cycles_left = 1000000000;

    for _ in 0..2 {
        configurations.clear();
        loop {
            for (x_values, y_values, horizontal, swap, next_top) in directions.iter() {
                simulate(x_values, y_values, &mut lines, *horizontal, *swap, *next_top);
            }
            cycles_left -= 1;
            if !configurations.insert(lines.clone()) {
                break;
            }
        }
    }


    cycles_left = cycles_left % configurations.len();

    for _ in 0..cycles_left {
        for (x_values, y_values, horizontal, swap, next_top) in directions.iter() {
            simulate(x_values, y_values, &mut lines, *horizontal, *swap, *next_top);
        }
    }

    println!("Second star: {}", total_load(&lines, width, height));
}

fn total_load(lines: &Grid, width: usize, height: usize) -> usize {
    let mut total_load: usize = 0;
    for x in 0..width {
        for y in 0..height {
            if lines[y][x] == 'O' {
                total_load += height - y;
            }
        }
    }
    total_load
}

fn simulate(x_values: &Vec<usize>, y_values: &Vec<usize>, lines: &mut Grid, horizontal: bool, swap: fn(&mut Grid, usize, usize, usize), next_top: fn(usize, usize) -> i64) {
    if horizontal {
        for y in y_values.iter() {
            let mut left: i64 = *x_values.first().unwrap() as i64;
            for x in x_values.iter() {
                left = maybe_swap(lines, *x, *y, left, swap, next_top);
            }
        }
    } else {
        for x in x_values.iter() {
            let mut top: i64 = *y_values.first().unwrap() as i64;
            for y in y_values.iter() {
                top = maybe_swap(lines, *x, *y, top, swap, next_top);
            }
        }
    }
}

fn maybe_swap(lines: &mut Grid, x: usize, y: usize, top: i64, swap: fn(&mut Grid, usize, usize, usize), next_top: fn(usize, usize) -> i64) -> i64 {
    return match lines[y][x] {
        '#' => next_top(x, y),
        'O' => {
            swap(lines, x, y, top as usize);
            next_top(top as usize, top as usize)
        }
        _ => top
    };
}

fn _display(grid: &Grid) {
    println!("====== GRID ======");
    for line in grid {
        for c in line {
            print!("{c}");
        }
        println!()
    }
}