use std::cmp::Reverse;
use std::collections::HashSet;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use priority_queue::PriorityQueue;
use crate::Direction::RIGHT;
use crate::Direction::UP;
use crate::Direction::LEFT;
use crate::Direction::DOWN;

type Coord = (i64, i64);

#[derive(Clone, Debug)]
struct State {
    position: Coord,
    heat_loss: u64,
    steps_taken: u8,
    direction: Direction,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.steps_taken.hash(state);
        self.direction.hash(state);
    }
}

impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.steps_taken == other.steps_taken && self.direction == other.direction
    }
}

impl Eq for State {}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Direction { UP, DOWN, LEFT, RIGHT }

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let weights: Vec<Vec<u64>> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error parsing line").chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
        .collect();
    let initial_state = State {
        position: (0, 0),
        heat_loss: 0,
        steps_taken: 0,
        direction: RIGHT,
    };
    let width = weights.first().unwrap().len();
    let height = weights.len();
    let goal = (width as i64 - 1, height as i64 - 1);

    let mut queue = PriorityQueue::new();
    queue.push(initial_state.clone(), Reverse(heuristic(&initial_state, goal)));
    let mut open_set = HashSet::from([initial_state.clone()]);

    while queue.peek().unwrap().0.position != goal {
        let state = queue.pop().unwrap().0;
        open_set.remove(&state);
        let neighbor_directions = [
            (state.direction, state.steps_taken + 1),
            (turn_left(state.direction), 1),
            (turn_right(state.direction), 1)
        ];
        for (direction, steps_taken) in neighbor_directions {
            if steps_taken > 3 {
                continue;
            }
            let neighbor_position = advance(state.position, direction, width, height);
            match neighbor_position {
                None => continue,
                Some(neighbor_position) => {
                    let weight = weights[neighbor_position.1 as usize][neighbor_position.0 as usize];
                    let neighbor = State {
                        position: neighbor_position,
                        heat_loss: state.heat_loss + weight,
                        steps_taken,
                        direction,
                    };
                    if open_set.insert(neighbor.clone()) {
                        let f_score = Reverse(neighbor.heat_loss + heuristic(&neighbor, goal));
                        queue.push(neighbor, f_score);
                    }
                }
            }
        }
    }

    let state = queue.pop().unwrap().0;
    println!("First star: {}", state.heat_loss);

    queue.clear();
    queue.push(initial_state.clone(), Reverse(heuristic(&initial_state, goal)));
    open_set.clear();
    open_set.insert(initial_state.clone());

    while !(queue.peek().unwrap().0.position == goal && queue.peek().unwrap().0.steps_taken >= 4) {
        let state = queue.pop().unwrap().0;
        open_set.remove(&state);
        let neighbor_directions = [
            (state.direction, state.steps_taken + 1),
            (turn_left(state.direction), 1),
            (turn_right(state.direction), 1)
        ];
        for (direction, steps_taken) in neighbor_directions {
            if (direction == state.direction && steps_taken > 10) || (direction != state.direction && state.steps_taken < 4) {
                continue;
            }
            let neighbor_position = advance(state.position, direction, width, height);
            match neighbor_position {
                None => continue,
                Some(neighbor_position) => {
                    let weight = weights[neighbor_position.1 as usize][neighbor_position.0 as usize];
                    let neighbor = State {
                        position: neighbor_position,
                        heat_loss: state.heat_loss + weight,
                        steps_taken,
                        direction,
                    };
                    if open_set.insert(neighbor.clone()) {
                        let f_score = Reverse(neighbor.heat_loss + heuristic(&neighbor, goal));
                        queue.push(neighbor, f_score);
                    }
                }
            }
        }
    }

    let state = queue.pop().unwrap().0;
    println!("Second star: {}", state.heat_loss);
}

fn advance(position: Coord, direction: Direction, width: usize, height: usize) -> Option<Coord> {
    let new_position = match direction {
        UP => (position.0, position.1 - 1),
        DOWN => (position.0, position.1 + 1),
        LEFT => (position.0 - 1, position.1),
        RIGHT => (position.0 + 1, position.1)
    };
    if new_position.0 < 0 || new_position.0 >= width as i64 || new_position.1 < 0 || new_position.1 >= height as i64 {
        return None;
    }
    return Some(new_position);
}

fn turn_left(direction: Direction) -> Direction {
    match direction {
        UP => LEFT,
        LEFT => DOWN,
        DOWN => RIGHT,
        RIGHT => UP,
    }
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        UP => RIGHT,
        RIGHT => DOWN,
        DOWN => LEFT,
        LEFT => UP,
    }
}

fn heuristic(state: &State, goal: Coord) -> u64 {
    let current_position = state.position;
    return current_position.0.abs_diff(goal.0) + current_position.1.abs_diff(goal.1);
}