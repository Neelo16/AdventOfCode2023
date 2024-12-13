use std::ops;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug)]
struct Coord(i64, i64);

impl ops::Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::AddAssign<Coord> for Coord {
    fn add_assign(&mut self, rhs: Coord) {
        *self = *self + rhs;
    }
}

impl ops::Mul<i64> for Coord {
    type Output = Coord;

    fn mul(self, rhs: i64) -> Self::Output {
        Coord(self.0 * rhs, self.1 * rhs)
    }
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Error parsing line"))
        .collect();

    let mut vertex = Coord(0, 0);
    let mut vertices = vec![vertex];
    let mut perimeter = 0;
    for line in &lines {
        let (path, _) = parse(line);
        let next_vertex = vertex + path;
        perimeter += (next_vertex.0 - vertex.0).abs_diff(next_vertex.1 - vertex.1);
        vertex = next_vertex;
        vertices.push(vertex);
    }
    let x_values: Vec<_> = vertices.iter().map(|Coord(x, _)| x).collect();
    let y_values: Vec<_> = vertices.iter().map(|Coord(_, y)| y).collect();
    let area = tie_shoelace(&x_values, &y_values).abs_diff(tie_shoelace(&y_values, &x_values)) / 2;
    println!("First star: {}", area + perimeter / 2 + 1);

    let mut perimeter = 0;
    vertices.clear();
    for line in &lines {
        let (_, hex) = parse(line);
        let path = hex_to_path(&hex);
        let next_vertex = vertex + path;
        perimeter += (next_vertex.0 - vertex.0).abs_diff(next_vertex.1 - vertex.1);
        vertex = next_vertex;
        vertices.push(vertex);
    }
    let x_values: Vec<_> = vertices.iter().map(|Coord(x, _)| x).collect();
    let y_values: Vec<_> = vertices.iter().map(|Coord(_, y)| y).collect();
    let area = tie_shoelace(&x_values, &y_values).abs_diff(tie_shoelace(&y_values, &x_values)) / 2;
    println!("Second star: {}", area + perimeter / 2 + 1);
}

fn hex_to_path(hex: &String) -> Coord {
    let distance = i64::from_str_radix(&hex[0..5], 16).unwrap();
    let direction = string_to_direction(["R", "D", "L", "U"][hex.chars().last().unwrap().to_string().parse::<usize>().unwrap()]);
    return direction * distance;
}

fn tie_shoelace(x_values: &Vec<&i64>, y_values: &Vec<&i64>) -> i64 {
    x_values[0..x_values.len() - 1].iter().zip(y_values[1..y_values.len()].iter()).map(|(&&x, &&y)| x * y).sum::<i64>()
}

fn parse(line: &String) -> (Coord, String) {
    let mut words = line.split_whitespace();
    let direction = string_to_direction(words.next().unwrap());
    let scalar: i64 = words.next().unwrap().parse().unwrap();
    let hex = words.next().unwrap().replace("(#", "").strip_suffix(")").unwrap().to_owned();
    return (direction * scalar, hex);
}

fn string_to_direction(direction: &str) -> Coord {
    match direction {
        "R" => Coord(1, 0),
        "L" => Coord(-1, 0),
        "U" => Coord(0, 1),
        "D" => Coord(0, -1),
        _ => panic!("Invalid direction found")
    }
}