use std::borrow::ToOwned;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(Debug)]
struct Workflow {
    label: String,
    rules: Vec<Rule>,
    else_label: String,
}

#[derive(Debug, Clone)]
struct Rule {
    category: String,
    op: char,
    value: i64,
    dest_label: String,
}

#[derive(Debug, Clone)]
struct Range {
    lower_limit: i64,
    upper_limit: i64,
}

impl Range {
    fn valid_value_count(&self) -> i64 {
        return self.upper_limit - self.lower_limit;
    }
}

impl Rule {
    fn matches(&self, part: &HashMap<String, i64>) -> Option<String> {
        let matches = match self.op {
            '>' => part[&self.category] > self.value,
            '<' => part[&self.category] < self.value,
            _ => panic!("Invalid op")
        };

        if matches {
            Some(self.dest_label.to_owned())
        } else {
            None
        }
    }
}

impl Workflow {
    fn new(s: &String) -> Workflow {
        let mut rules = vec![];
        let re = Regex::new(r"(\w+)\{((?:[xmas][<>]\d+:\w+,)+)(\w+)}").unwrap();
        let (_, captures) = re.captures(s.as_str()).unwrap().extract::<3>();
        let label = captures[0];
        let rules_str = captures[1];
        let else_label = captures[2];
        let rule_regex = Regex::new(r"([xmas])([<>])(\d+):(\w+),").unwrap();
        for rule_match in rule_regex.captures_iter(rules_str) {
            let (_, [category, op_str, value, dest]) = rule_match.extract::<4>();
            let op = op_str.chars().next().unwrap();
            rules.push(Rule {
                category: category.to_owned(),
                op,
                value: value.parse::<i64>().unwrap(),
                dest_label: dest.to_owned(),
            });
        }

        return Workflow { label: label.to_owned(), rules, else_label: else_label.to_owned() };
    }

    fn transition(&self, part: &HashMap<String, i64>) -> String {
        for rule in &self.rules {
            match rule.matches(part) {
                None => {}
                Some(workflow) => return workflow
            }
        }
        return self.else_label.to_owned();
    }
}

const ACCEPT: &str = "A";
const REJECT: &str = "R";
const START: &str = "in";

fn paths_to_acceptance(workflow: &Workflow, workflow_map: &HashMap<String, Workflow>, mut reject_part: HashMap<String, Range>) -> Vec<HashMap<String, Range>> {
    let mut result = vec![];

    if workflow.label == ACCEPT {
        return vec![reject_part];
    } else if workflow.label == REJECT {
        return vec![];
    }


    for rule in &workflow.rules {
        let mut accept_part = reject_part.clone();
        let existing_constraint = &reject_part[&rule.category];
        match rule.op {

            '>' => {
                accept_part.insert(rule.category.clone(), Range {
                    lower_limit: rule.value + 1,
                    upper_limit: existing_constraint.upper_limit,
                });
                reject_part.insert(rule.category.clone(), Range {
                    lower_limit: existing_constraint.lower_limit,
                    upper_limit: rule.value + 1
                })
            },
            '<' => {
                accept_part.insert(rule.category.clone(), Range {
                    lower_limit: existing_constraint.lower_limit,
                    upper_limit: rule.value
                });
                reject_part.insert(rule.category.clone(), Range {
                    lower_limit: rule.value,
                    upper_limit: existing_constraint.upper_limit,
                })
            },
            _ => panic!("Invalid op")
        };

        result.extend(paths_to_acceptance(&workflow_map[&rule.dest_label], workflow_map, accept_part));
    }

    result.extend(paths_to_acceptance(&workflow_map[&workflow.else_label], workflow_map, reject_part.clone()));

    return result;
}

fn new_part(s: &String) -> HashMap<String, i64> {
    let re = Regex::new(r"([xmas])=(\d+)").unwrap();
    let mut part = HashMap::new();
    for category_match in re.captures_iter(s.as_str()) {
        let (_, [category, value]) = category_match.extract::<2>();
        part.insert(category.to_owned(), value.parse::<i64>().unwrap());
    }
    return part;
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error parsing line"))
        .collect();

    let mut lines_iter = lines.iter().peekable();
    let mut workflows = HashMap::new();
    let mut parts = vec![];

    while !lines_iter.peek().unwrap().is_empty() {
        let workflow = Workflow::new(lines_iter.next().unwrap());
        workflows.insert(workflow.label.to_owned(), workflow);
    }

    for label in [ACCEPT, REJECT] {
        workflows.insert(label.to_owned(), Workflow {
            label: label.to_owned(),
            rules: vec![],
            else_label: "".to_string(),
        });
    }

    lines_iter.next();

    while lines_iter.peek().is_some() {
        parts.push(new_part(lines_iter.next().unwrap()));
    }

    let mut result: i64 = 0;

    let initial_workflow = &workflows[START];
    let terminal_states: [String; 2] = [String::from(ACCEPT), String::from(REJECT.to_string())];

    for part in &parts {
        let mut current_workflow = initial_workflow;
        while !terminal_states.contains(&current_workflow.label) {
            current_workflow = &workflows[&current_workflow.transition(part)];
        }
        if current_workflow.label == ACCEPT {
            result += part.values().sum::<i64>();
        }
    }

    println!("First star: {result}");


    let constraints = paths_to_acceptance(initial_workflow, &workflows, HashMap::from([
        ("x".to_owned(), Range { lower_limit: 1, upper_limit: 4001 }),
        ("m".to_owned(), Range { lower_limit: 1, upper_limit: 4001 }),
        ("a".to_owned(), Range { lower_limit: 1, upper_limit: 4001 }),
        ("s".to_owned(), Range { lower_limit: 1, upper_limit: 4001 }),
    ]));

    let result = constraints.iter().map(|p| p.values().fold(1, |acc, c| acc * c.valid_value_count())).sum::<i64>();
    println!("Second star: {}", result);
}
