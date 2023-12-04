use std::fs;

struct Set {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

fn parse_set(s: &str) -> Set {
    let mut set = Set {
        red: None,
        green: None,
        blue: None,
    };
    for cubes in s.split(',') {
        let mut cubes = cubes.split_whitespace();
        let amount: u32 = cubes.next().unwrap().parse().unwrap();
        let color = cubes.next().unwrap().trim();
        match color {
            "red" => set.red = Some(amount),
            "green" => set.green = Some(amount),
            "blue" => set.blue = Some(amount),
            _ => {}
        }
    }
    set
}

fn is_set_valid(limits: &Set, set: Set) -> bool {
    if (set.red > limits.red) | (set.green > limits.green) | (set.blue > limits.blue) {
        return false;
    }
    true
}

fn parse_game(limits: &Set, game: &str) -> u32 {
    let mut parts = game.split(':');
    let id: u32 = parts
        .next()
        .unwrap()
        .strip_prefix("Game ")
        .unwrap()
        .parse()
        .unwrap();
    let body = parts.next().unwrap();
    let is_valid = body
        .split(';')
        .map(parse_set)
        .all(|s| is_set_valid(&limits, s));
    match is_valid {
        true => id,
        false => 0,
    }
}

fn main() {
    let limits = Set {
        red: Some(12),
        green: Some(13),
        blue: Some(14),
    };
    let file = fs::read_to_string("./input.txt").unwrap();
    let res: u32 = file.lines().map(|l| parse_game(&limits, l)).sum();
    println!("{res}");
}
