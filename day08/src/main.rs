use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use gcd::Gcd;

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error parsing line"))
        .collect();
    let mut lines_iter = lines.iter();
    let directions: Vec<_> = lines_iter.next().unwrap().chars().map(|c| {
        match c {
            'L' => |t: &(String, String)| t.0.to_string(),
            'R' => |t: &(String, String)| t.1.to_string(),
            _ => panic!("Unexpected direction"),
        }
    })
        .collect();
    lines_iter.next();
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    for line in lines_iter {
        let mut words = line.split_whitespace();
        let node = words.next().unwrap().to_string();
        assert_eq!("=", words.next().unwrap());
        let paths = (
            words.next().unwrap().replace("(", "").strip_suffix(",").unwrap().to_string(),
            words.next().unwrap().strip_suffix(")").unwrap().to_string()
        );
        nodes.insert(node, paths);
    }

    let mut current_node = "AAA".to_string();
    let dest = "ZZZ";
    let mut counter = 0;
    let mut directions_iter = directions.iter().cycle();

    while current_node != dest {
        current_node = directions_iter.next().unwrap()(&nodes[&current_node]);
        counter += 1;
    }
    println!("First star: {}", counter);

    let start_nodes: Vec<_> = nodes.keys().filter(|s| s.ends_with("A")).map(|s| s.to_string()).collect();
    let mut cycles: Vec<_> = vec![];

    for node in &start_nodes {
        let mut current_node = node.to_string();
        let mut direction_indices_iter = (0..directions.len()).cycle().peekable();
        directions_iter = directions.iter().cycle();
        let mut cycle_start: u64 = 0;
        while !current_node.ends_with("Z") {
            current_node = directions_iter.next().unwrap()(&nodes[&current_node]);
            direction_indices_iter.next();
            cycle_start += 1;
        }
        let mut cycle_length: u64 = 0;
        let mut visited_nodes: HashSet<(usize, String)> = HashSet::new();
        while !visited_nodes.contains(&(*direction_indices_iter.peek().unwrap(), current_node.to_string())) {
            visited_nodes.insert((direction_indices_iter.next().unwrap(), current_node.to_string()));
            current_node = directions_iter.next().unwrap()(&nodes[&current_node]);
            cycle_length += 1;
        }
        cycles.push((cycle_start, cycle_length));
    }

    if cycles.iter().all(|(start, period)| start == period) {
        let lcm = cycles.iter().map(|(_, period)| *period).reduce(|acc, p| lcm(acc, p)).unwrap();
        println!("Second star: {}", lcm);
    } else {
        let mut found = false;
        while !found {
            let (i, (smallest_pos, cycle_length)) = cycles.iter().enumerate().min_by(|(_, (pos, _)), (_, (pos2, _))| pos.cmp(pos2)).unwrap();
            cycles[i] = (smallest_pos + cycle_length, *cycle_length);
            let different_indices: HashSet<_> = cycles.iter().map(|(pos, _)| pos).collect();
            found = different_indices.len() == 1;
        }

        println!("Second star: {}", cycles.first().unwrap().0);
    }
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / first.gcd(second)
}