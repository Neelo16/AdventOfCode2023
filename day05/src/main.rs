use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

struct Almanac {
    seeds: Vec<i64>,
    seed_ranges: Vec<Range<i64>>,
    mappings: HashMap<String, Vec<Mapping>>,
    categories: Vec<String>,
}

struct Mapping {
    src: i64,
    dst: i64,
    range: i64,
    dst_category: String,
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error parsing line"))
        .collect();

    let almanac = parse_almanac(&lines);

    println!("First star: {}", almanac
        .seeds
        .iter()
        .map(|seed| calculate_destination_value(seed, &almanac, false))
        .min()
        .unwrap());

    let mut location: i64 = 0;
    let inverted_almanac = invert(&almanac);

    while !valid_seed(calculate_destination_value(&location, &inverted_almanac, false), &inverted_almanac) {
        location += 1;
    }
    println!("Second star: {}", location);
    calculate_destination_value(&location, &inverted_almanac, true);
}

fn valid_seed(number: i64, almanac: &Almanac) -> bool {
    let seed = almanac.seed_ranges.iter().find(|sr| sr.contains(&number));
    return seed.is_some();
}

fn parse_almanac(lines: &Vec<String>) -> Almanac {
    let mut lines_iter = lines.iter().peekable();

    let seeds = parse_seeds(lines_iter.peek().unwrap());
    let seed_ranges = parse_seed_ranges(lines_iter.next().unwrap());
    let mut mappings = HashMap::new();
    let mut categories = vec![String::from("seed")];

    while lines_iter.next().is_some() {
        let mut mapping_categories = lines_iter
            .next()
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap()
            .split("-");
        let src_category = mapping_categories.next().unwrap();
        assert_eq!("to", mapping_categories.next().unwrap());
        let dst_category = mapping_categories.next().unwrap();
        let mut category_mappings = vec![];

        while lines_iter.peek().is_some() && !lines_iter.peek().unwrap().is_empty() {
            let mut words = lines_iter.next().unwrap().split_whitespace();
            let dst: i64 = words.next().unwrap().parse().unwrap();
            let src: i64 = words.next().unwrap().parse().unwrap();
            let range: i64 = words.next().unwrap().parse().unwrap();
            category_mappings.push(Mapping { src, dst, range, dst_category: dst_category.to_string() });
        }

        mappings.insert(src_category.to_string(), category_mappings);
        categories.push(dst_category.to_string());
    }

    return Almanac { seeds, seed_ranges, mappings, categories };
}

fn parse_seeds(line: &String) -> Vec<i64> {
    let mut words = line.split_whitespace().peekable();
    assert_eq!(Some("seeds:"), words.next());
    return words.map(|w| w.parse().unwrap()).collect();
}

fn parse_seed_ranges(line: &String) -> Vec<Range<i64>> {
    let mut words = line.split_whitespace().peekable();
    assert_eq!(Some("seeds:"), words.next());
    let mut seeds = vec![];
    while words.peek().is_some() {
        let start: i64 = words.next().unwrap().parse().unwrap();
        let range: i64 = words.next().unwrap().parse().unwrap();
        seeds.push(start..start + range);
    }
    return seeds;
}

fn calculate_destination_value(seed: &i64, almanac: &Almanac, debug: bool) -> i64 {
    let mut categories_iter = almanac.categories.iter();
    let mut category = categories_iter.next().unwrap();
    let mut current_number = *seed;
    let destination = almanac.categories.last().unwrap();
    if debug {
        print!("{} ", current_number)
    }
    while category != destination {
        let mapping = almanac.mappings[category]
            .iter()
            .find(|m| Range { start: m.src, end: m.src + m.range }.contains(&current_number));
        category = categories_iter.next().unwrap();
        match mapping {
            Some(m) => {
                current_number = m.dst + current_number - m.src;
                assert_eq!(category, &m.dst_category);
            }
            None => {
                current_number = current_number;
            }
        }
        if debug {
            print!("-> {} ", current_number)
        }
    }
    if debug { println!() }
    return current_number;
}

fn invert(almanac: &Almanac) -> Almanac {
    let mut reversed_mappings = HashMap::new();
    let mut reversed_categories = almanac.categories.clone();
    reversed_categories.reverse();
    let mut i: usize = 0;
    while i < reversed_categories.len() - 1 {
        let category = &reversed_categories[i].to_string();
        let dst_category = reversed_categories[i + 1].to_string();
        reversed_mappings.insert(category.to_string(), almanac
            .mappings[&dst_category]
            .iter()
            .map(|m| Mapping {
                src: m.dst,
                dst: m.src,
                range: m.range,
                dst_category: dst_category.to_string(),
            } )
            .collect());
        i += 1;
    }
    return Almanac { seeds: almanac.seeds.clone(), seed_ranges: almanac.seed_ranges.clone(), mappings: reversed_mappings, categories: reversed_categories };
}