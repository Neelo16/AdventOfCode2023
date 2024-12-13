use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Grid = Vec<Vec<char>>;
type Coord = (i64, i64);

#[derive(Clone, Copy, Debug, Eq, PartialOrd, PartialEq, Hash)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let lines: Grid = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error parsing line").chars().collect())
        .collect();
    println!("First star: {}", propagate(&lines, (0, 0), Direction::RIGHT));

    let mut max_propagation: usize = 0;
    let height = lines.len();
    let width = lines.first().unwrap().len();
    for y in 0..height {
        max_propagation = max_propagation.max(propagate(&lines, (0, y as i64), Direction::RIGHT));
        max_propagation = max_propagation.max(propagate(&lines, ((width - 1) as i64, y as i64), Direction::LEFT));
    }
    for x in 0..width {
        max_propagation = max_propagation.max(propagate(&lines, (x as i64, 0), Direction::DOWN));
        max_propagation = max_propagation.max(propagate(&lines, (x as i64, (height - 1) as i64), Direction::UP));
    }
    println!("Second star: {max_propagation}");
}

fn propagate(grid: &Grid, start: Coord, start_direction: Direction) -> usize {
    let mut coords = HashSet::new();
    let width = grid.first().unwrap().len();
    let height = grid.len();
    let mut coord_queue = VecDeque::from([(start, start_direction)]);
    let mut visited = HashSet::new();
    while !coord_queue.is_empty() {
        let (coord, direction) = coord_queue.pop_front().unwrap();
        if !visited.insert((coord, direction)) {
            continue;
        }
        coords.insert(coord);
        let space = grid[coord.1 as usize][coord.0 as usize];
        let directions = match space {
            '.' => vec![direction],
            '\\' | '/' => vec![reflection_direction(direction, space)],
            '-' | '|' => splitter_directions(direction, space),
            _ => panic!()
        };
        for next_direction in directions {
            let next_coord = movement(coord, next_direction, width, height);
            match next_coord {
                None => {}
                Some(coord) => coord_queue.push_back((coord, next_direction))
            }
        }
    }
    return coords.len();
}

fn movement(start: Coord, direction: Direction, width: usize, height: usize) -> Option<Coord> {
    let coord = match direction {
        Direction::UP => {
            (start.0, start.1 - 1)
        }
        Direction::DOWN => {
            (start.0, start.1 + 1)
        }
        Direction::RIGHT => {
            (start.0 + 1, start.1)
        }
        Direction::LEFT => {
            (start.0 - 1, start.1)
        }
    };

    return if coord.0 < 0 || coord.0 >= width as i64 || coord.1 < 0 || coord.1 >= height as i64 {
        None
    } else {
        Some(coord)
    };
}

fn reflection_direction(direction: Direction, mirror: char) -> Direction {
    return match mirror {
        '\\' => {
            match direction {
                Direction::UP => Direction::LEFT,
                Direction::DOWN => Direction::RIGHT,
                Direction::RIGHT => Direction::DOWN,
                Direction::LEFT => Direction::UP
            }
        }
        '/' => {
            match direction {
                Direction::UP => Direction::RIGHT,
                Direction::DOWN => Direction::LEFT,
                Direction::RIGHT => Direction::UP,
                Direction::LEFT => Direction::DOWN
            }
        }
        _ => panic!()
    };
}

fn splitter_directions(direction: Direction, splitter: char) -> Vec<Direction> {
    return match splitter {
        '|' => match direction {
            Direction::UP | Direction::DOWN => vec![direction],
            Direction::RIGHT | Direction::LEFT => vec![Direction::UP, Direction::DOWN],
        },
        '-' => match direction {
            Direction::UP | Direction::DOWN => vec![Direction::LEFT, Direction::RIGHT],
            Direction::RIGHT | Direction::LEFT => vec![direction]
        },
        _ => panic!()
    };
}

fn _display(grid: &Grid, energized_tiles: &HashSet<Coord>) {
    for (y, line) in grid.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if energized_tiles.contains(&(x as i64, y as i64)) {
                print!("#")
            } else {
                print!("{}", char)
            }
        }
        println!()
    }
}