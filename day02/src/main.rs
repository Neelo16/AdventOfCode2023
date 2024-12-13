use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::borrow::Borrow;

fn get_max_values_for_game(line: &String) -> (i32, HashMap<String, i32>) {
    let mut words = line.split_whitespace().into_iter().peekable();
    assert_eq!(words.next(), Some("Game"));
    let id: i32 = words.next().unwrap()
        .strip_suffix(":").expect("Missing colon after game id")
        .parse().expect("Failed to parse id");
    let mut parsed_line: HashMap<String, i32> = HashMap::new();

    while words.peek().is_some() {
        let count: i32 = words
            .next().unwrap()
            .parse().expect("Failed to parse count");
        let color = words
            .next().expect("Missing color after count")
            .trim_end_matches(|c| c == ';' || c == ',');
        let current_max_value = parsed_line.get(color).or(Some(0_i32.borrow())).unwrap();
        if current_max_value < &count {
            parsed_line.insert(String::from(color), count);
        }
    }

    return (id, parsed_line);
}

fn is_game_possible(game: &HashMap<String, i32>) -> bool {
    let expected_max_values = HashMap::from([
        (String::from("red"), 12),
        (String::from("green"), 13),
        (String::from("blue"), 14)
    ]);

    return game.iter().all(|(color, count)| {
        let max_value = *expected_max_values.get(color).or(Some(0_i32.borrow())).unwrap();
        count <= &max_value
    });
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error parsing line"))
        .collect();

    let id_and_max_values_for_games: Vec<(i32, HashMap<String, i32>)> =
        lines.iter()
            .map(get_max_values_for_game)
            .collect();
    println!("First star: {}",
             id_and_max_values_for_games
                 .iter()
                 .filter(|(_, max_values)| is_game_possible(max_values))
                 .map(|(id, _)| id)
                 .sum::<i32>());
    println!("Second star: {}",
    id_and_max_values_for_games
        .iter()
        .map(|(_, max_values)|
            max_values.values().map(|value| *value)
            .reduce(|acc, count| acc * count).or(Some(0)).unwrap()
        )
        .sum::<i32>()
    );
}
