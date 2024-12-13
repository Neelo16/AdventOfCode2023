use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = (usize, usize);

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error parsing line"))
        .collect();
    let mut transitions: HashMap<Coord, Vec<Coord>> = HashMap::new();
    let mut start_pos: Coord = (0, 0);
    let map_size = (lines.first().unwrap().len(), lines.len());
    let tiles: Vec<Vec<char>> = lines
        .iter()
        .map(|l| l.chars().collect())
        .collect();

    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let mut edges: Vec<Coord> = vec![];
            match char {
                '|' => {
                    if y > 0 {
                        edges.push((x, y - 1))
                    }
                    edges.push((x, y + 1))
                }
                '-' => {
                    if x > 0 {
                        edges.push((x - 1, y));
                    }
                    edges.push((x + 1, y))
                }
                'L' => {
                    if y > 0 {
                        edges.push((x, y - 1));
                    }
                    edges.push((x + 1, y))
                }
                'J' => {
                    if x > 0 {
                        edges.push((x - 1, y))
                    }
                    if y > 0 {
                        edges.push((x, y - 1));
                    }
                }
                '7' => {
                    if x > 0 {
                        edges.push((x - 1, y))
                    }
                    edges.push((x, y + 1))
                }
                'F' => {
                    edges.push((x, y + 1));
                    edges.push((x + 1, y))
                }
                _ => {}
            };
            transitions.insert((x, y), edges);
            if char == 'S' {
                start_pos = (x, y);
            }
        }
    }

    let (start_x, start_y) = (start_pos.0 as i64, start_pos.1 as i64);
    transitions.insert(start_pos,
                       [
                           (start_x + 1, start_y),
                           (start_x - 1, start_y),
                           (start_x, start_y + 1),
                           (start_x, start_y - 1),
                       ]
                           .iter()
                           .map(|(x, y)| (*x as usize, *y as usize))
                           .filter(|c| transitions.get(c).or(Some(&vec![])).unwrap().contains(&start_pos))
                           .collect()
    );

    let (steps, contained) = solve(start_pos, &transitions, map_size, &tiles);
    println!("First star: {}", steps);
    println!("Second star: {}", contained);
}

fn solve(start: (usize, usize), transitions: &HashMap<Coord, Vec<Coord>>, map_size: (usize, usize), tiles: &Vec<Vec<char>>) -> (u64, u64) {
    let mut visited: HashSet<Coord> = HashSet::from([start]);
    let mut node_queue: VecDeque<(u64, Coord)> = VecDeque::from_iter(transitions[&start].iter().map(|c| (1, *c)));
    assert_eq!(node_queue.len(), 2);
    let mut max_distance = 0;
    while !node_queue.is_empty() {
        let (distance, next_coord) = node_queue.pop_front().unwrap();
        max_distance = max(max_distance, distance);
        visited.insert(next_coord);
        for edge in &transitions[&next_coord] {
            if !visited.contains(edge) {
                node_queue.push_back((distance + 1, *edge));
            }
        }
    }

    let mut tiles_enclosed = 0;
    let (width, height) = map_size;
    for y in 0..height {
        let mut within_loop = false;
        for x in 0..width {
            if visited.contains(&(x, y)) {
                if is_obstacle(tiles[y][x]) {
                    within_loop = !within_loop;
                }
            } else if within_loop {
                tiles_enclosed += 1;
            }
        }
    }

    return (max_distance, tiles_enclosed);
}

fn is_obstacle(tile: char) -> bool {
    return ['|', 'L', 'J'].contains(&tile)
}
