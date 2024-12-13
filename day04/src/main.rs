use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Card {
    card_number: usize,
    actual_numbers: Vec<i64>,
    winning_numbers: Vec<i64>,
}

fn parse_line(line: &String) -> Card {
    let mut actual_numbers = vec![];
    let mut winning_numbers = vec![];
    let mut words = line.split_whitespace().peekable();
    assert_eq!(Some("Card"), words.next());
    let card_number = words.next().unwrap().strip_suffix(":").unwrap().parse::<usize>().unwrap();
    while *words.peek().unwrap() != "|" {
        winning_numbers.push(words.next().unwrap().parse::<i64>().unwrap());
    }
    words.next();
    while words.peek().is_some() {
        actual_numbers.push(words.next().unwrap().parse::<i64>().unwrap());
    }
    actual_numbers.sort();
    winning_numbers.sort();
    return Card { card_number, actual_numbers, winning_numbers };
}

fn count_winning_numbers(card: &Card) -> usize {
    let mut actual_index: usize = 0;
    let mut winning_index: usize = 0;
    let mut winning_count = 0;
    while actual_index < card.actual_numbers.len() && winning_index < card.winning_numbers.len() {
        while actual_index < card.actual_numbers.len() && card.actual_numbers[actual_index] < card.winning_numbers[winning_index] {
            actual_index += 1;
        }
        if actual_index < card.actual_numbers.len() {
            while winning_index < card.winning_numbers.len() && card.winning_numbers[winning_index] < card.actual_numbers[actual_index] {
                winning_index += 1;
            }
        }
        if actual_index < card.actual_numbers.len() && card.actual_numbers.get(actual_index) == card.winning_numbers.get(winning_index) {
            winning_count += 1;
            actual_index += 1;
            winning_index += 1;
        }
    }
    return winning_count;
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error parsing line"))
        .collect();
    let cards: Vec<Card> = lines.iter().map(parse_line).collect();
    let counts: Vec<usize> = cards.iter().map(count_winning_numbers).collect();
    println!("First star: {}", counts.iter().map(|c| if *c == 0 { 0 } else { 1 << (c - 1) } ).sum::<i64>());

    let mut processed_card_count = cards.len();
    let mut cards_to_process = VecDeque::with_capacity(cards.len() * 2);
    for card in &cards {
        cards_to_process.push_back(card);
    }

    while !cards_to_process.is_empty() {
        let card = &cards_to_process.pop_front().unwrap();
        let winning_number_count = counts[card.card_number - 1];
        processed_card_count += winning_number_count;
        for card in &cards[card.card_number..card.card_number + winning_number_count] {
            cards_to_process.push_back(card);
        }
    }
    println!("Second star: {}", processed_card_count);
}
