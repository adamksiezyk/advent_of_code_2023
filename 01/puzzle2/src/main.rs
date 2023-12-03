use std::collections::HashMap;
use std::fs;

fn decode_first(line: &str, mapping: &HashMap<&str, u32>) -> u32 {
    let mut buff: String = String::new();
    for c in line.chars() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
        buff.push(c);
        let key = mapping
            .keys()
            .filter(|k| buff.contains(k.to_owned()))
            .next();
        match key {
            Some(k) => {
                return mapping.get(k).unwrap().to_owned();
            }
            None => {}
        }
    }
    0
}

fn decode_last(line: &str, mapping: &HashMap<&str, u32>) -> u32 {
    let mut buff: String = String::new();
    for c in line.chars().rev() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
        buff.push(c);
        let key = mapping
            .keys()
            .filter(|k| buff.contains(&reverse(k.to_owned())))
            .next();
        match key {
            Some(k) => {
                return mapping.get(k).unwrap().to_owned();
            }
            None => {}
        }
    }
    0
}

fn reverse(string: &str) -> String {
    string.chars().rev().collect::<String>()
}

fn main() {
    let mapping: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let file = fs::read_to_string("./input.txt").unwrap();
    let result: u32 = file
        .lines()
        .map(|l| 10 * decode_first(l, &mapping) + decode_last(l, &mapping))
        .sum();

    println!("{}", result);
}
