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

fn get_min_required_set(prev_min: Set, set: Set) -> Set {
    let mut min_required_set = Set { ..prev_min };
    if set.red > prev_min.red {
        min_required_set.red = set.red;
    }
    if set.green > prev_min.green {
        min_required_set.green = set.green;
    }
    if set.blue > prev_min.blue {
        min_required_set.blue = set.blue;
    }

    min_required_set
}

fn parse_game(game: &str) -> u32 {
    let mut parts = game.split(':');
    let _: u32 = parts
        .next()
        .unwrap()
        .strip_prefix("Game ")
        .unwrap()
        .parse()
        .unwrap();
    let body = parts.next().unwrap();
    let min_required = body
        .split(';')
        .map(parse_set)
        .reduce(get_min_required_set)
        .unwrap();
    min_required.red.unwrap_or(0) * min_required.green.unwrap_or(0) * min_required.blue.unwrap_or(0)
}

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let res: u32 = file.lines().map(parse_game).sum();
    println!("{res}");
}
