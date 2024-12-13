use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Bet {
    hand: String,
    bid: i64,
    score: i64
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error parsing line"))
        .collect();
    let mut bets = vec![];
    for line in lines {
        let mut words = line.split_whitespace();
        bets.push(Bet{ hand: words.next().unwrap().to_string(), bid: words.next().unwrap().parse().unwrap(), score: 0 })
    }

    for bet in bets.iter_mut() {
        bet.score = score(&bet.hand, false);
    }
    bets.sort_by(|b1, b2| b1.score.cmp(&b2.score));
    println!("First star: {}", bets.iter().enumerate().map(|(i, bet)| bet.bid * (i as i64 + 1)).sum::<i64>());
    for bet in bets.iter_mut() {
        bet.score = score(&bet.hand, true);
    }
    bets.sort_by(|b1, b2| b1.score.cmp(&b2.score));
    println!("Second star: {}", bets.iter().enumerate().map(|(i, bet)| bet.bid * (i as i64 + 1)).sum::<i64>());
}

fn get_sorted_counts(hand: &String, joker: bool) -> Vec<i32> {
    let mut count_map: HashMap<char, i32> = HashMap::new();
    for card in hand.chars() {
        count_map.insert(card, *count_map.get(&card).get_or_insert(&0) + 1);
    }
    let joker_count = **count_map.get(&'J').get_or_insert(&0);
    if joker {
        count_map.remove(&'J');
    }
    let mut counts: Vec<i32> = count_map.values().map(|v| *v).collect();
    counts.sort_by(|i1, i2| i2.cmp(i1));
    if joker {
        if counts.is_empty() {
            counts.push(joker_count);
        } else {
            counts[0] += joker_count;
        }
    }
    return counts;
}

fn score(hand: &String, joker: bool) -> i64 {
    let counts = get_sorted_counts(hand, joker);
    let pow = match &counts[..] {
        [5] => 6,
        [4, 1] => 5,
        [3, 2] => 4,
        [3, 1, 1] => 3,
        [2, 2, 1] => 2,
        [2, ..] => 1,
        _ => 0
    };
    let mut score = 10_i64.pow(pow);
    let card_scores: HashMap<char, usize> = if joker {
        "0J123456789TQKA"
    } else {
        "0123456789TJQKA"
    }
        .chars().enumerate().map(|(i, c)| (c, i)).collect();
    for card in hand.chars() {
        score = score*100 + card_scores[&card] as i64;
    }
    return score;
}