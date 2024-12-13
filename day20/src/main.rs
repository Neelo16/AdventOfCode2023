use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use gcd::Gcd;
use regex::Regex;
use crate::ComponentType::{Broadcaster, Conjunction, FlipFlop};

#[derive(Clone)]
struct Node {
    name: String,
    neighbors: Vec<String>,
    component_type: ComponentType,
    memory: HashMap<String, bool>,
    state: bool,
}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.memory == other.memory && self.state == other.state
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("name", &self.name)
            .field("state", if self.state { &0 } else { &1 })
            .finish()
    }
}

impl Eq for Node {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ComponentType {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Signal {
    source: String,
    target: String,
    pulse: bool,
}


impl Signal {
    fn process(&self, graph: &mut HashMap<String, Node>, emissions: &mut HashMap<String, bool>) -> Vec<Signal> {
        let source = graph.get(&self.source).and_then(|g| Some(g.clone()));
        match graph.get_mut(&self.target) {
            None => vec![],
            Some(target) => {
                let pulse = match target.component_type {
                    Broadcaster => self.pulse,
                    Conjunction => {
                        target.memory.insert(self.source.to_owned(), self.pulse);
                        target.memory.values().any(|&p| !p)
                    }
                    FlipFlop => {
                        if !self.pulse {
                            target.state = !target.state;
                        }
                        target.state
                    }
                };

                if self.pulse && target.component_type == FlipFlop {
                    return vec![];
                }

                emissions.insert(self.source.to_owned(), pulse);

                return target.neighbors.iter()
                    .map(|n| {
                        Signal { source: self.target.to_owned(), target: n.to_owned(), pulse }
                    })
                    .collect();
            }
        }
    }
}

fn parse(line: &String) -> Node {
    let re = Regex::new(r"(broadcaster|[&%]\w+) -> ((?:\w+, )*)(\w+)").unwrap();
    let capture = re.captures_iter(line.as_str()).next().unwrap();
    let (_, [node, neighbors, final_neighbor]) = capture.extract::<3>();
    let (name, component_type) = match node.chars().next().unwrap() {
        'b' => (node.to_owned(), Broadcaster),
        '%' => (node[1..node.len()].to_owned(), FlipFlop),
        '&' => (node[1..node.len()].to_owned(), Conjunction),
        _ => panic!("Invalid node type")
    };
    let mut all_neighbors = vec![];
    for capture in Regex::new(r"(\w+), ").unwrap().captures_iter(neighbors) {
        let (_, [neighbor]) = capture.extract::<1>();
        all_neighbors.push(neighbor.to_owned());
    }
    all_neighbors.push(final_neighbor.to_owned());
    return Node {
        name,
        neighbors: all_neighbors,
        component_type,
        memory: HashMap::new(),
        state: false,
    };
}

fn lcm(a: u64, b: u64) -> u64 {
    return a * b / a.gcd(b);
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error parsing line"))
        .collect();
    let mut graph = HashMap::new();
    let mut conjunction_nodes = HashMap::new();
    for line in &lines {
        let node = parse(line);
        if node.component_type == Conjunction {
            conjunction_nodes.insert(node.name.to_owned(), vec![]);
        }
        graph.insert(node.name.to_owned(), node);
    }

    for node in graph.values() {
        for neighbor in &node.neighbors {
            let node_inputs = conjunction_nodes.get_mut(neighbor);
            match node_inputs {
                Some(v) => v.push(node.name.to_owned()),
                None => {}
            }
        }
    }

    for (conjunction_node, inputs) in &conjunction_nodes {
        let node = graph.get_mut(conjunction_node).unwrap();
        for input in inputs {
            node.memory.insert(input.to_owned(), false);
        }
    }

    let initial_graph = graph.clone();

    let mut signal_queue = VecDeque::new();

    let button_signal = Signal {
        source: "button".to_owned(),
        target: "broadcaster".to_owned(),
        pulse: false,
    };

    let mut low_count = 0;
    let mut high_count = 0;
    let mut emissions: HashMap<String, bool> = HashMap::new();

    for _ in 0..1000 {
        signal_queue.push_back(button_signal.clone());
        low_count += 1;
        while !signal_queue.is_empty() {
            let signal = signal_queue.pop_front().unwrap();
            let new_signals = signal.process(&mut graph, &mut emissions);
            let high_pulses = new_signals.iter().filter(|s| s.pulse).count();
            high_count += high_pulses;
            low_count += new_signals.len() - high_pulses;
            signal_queue.extend(new_signals)
        }
    }

    println!("First star: {}", low_count * high_count);

    graph = initial_graph;

    let mut found = false;
    let mut count: u64 = 0;

    let mut conjunction_states: HashMap<String, (bool, u64)> = HashMap::from([
        ("bx".to_owned(), (false, 0)),
        ("gj".to_owned(), (false, 0)),
        ("qq".to_owned(), (false, 0)),
        ("bc".to_owned(), (false, 0)),
    ]);

    let mut bx: u64 = 0;
    let mut qq: u64 = 0;
    let mut bc: u64 = 0;
    let mut gj: u64 = 0;

    emissions.clear();

    while !found {
        signal_queue.push_back(button_signal.clone());
        count += 1;
        while !signal_queue.is_empty() {
            let signal = signal_queue.pop_front().unwrap();
            let new_signals = signal.process(&mut graph, &mut emissions);
            if new_signals.iter().any(|s| !s.pulse && s.target == "rx") {
                found = true;
                break;
            }
            signal_queue.extend(new_signals);

            if bx <= 0 && emissions.get("bx").is_some_and(|v| *v) {
                bx = count;
            }
            if qq <= 0 && emissions.get("qq").is_some_and(|v| *v) {
                qq = count;
            }
            if gj == 0 && emissions.get("gj").is_some_and(|v| *v)  {
                gj = count;
            }
            if bc == 0 && emissions.get("bc").is_some_and(|v| *v) {
                bc = count;
            }

            found = [bx, qq, gj, bc].iter().all(|&v| v != 0);
        }
    }

    println!("Second star: {}", bx * qq * gj * bc);
}
