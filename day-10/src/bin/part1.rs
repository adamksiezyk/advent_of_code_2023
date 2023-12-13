use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Tile {
    label: char,
    coords: Point,
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let output = part1(input);
    dbg!(output);
}

fn part1(input: String) -> i32 {
    let tiles = parse_tiles(input);
    let start = tiles
        .iter()
        .find_map(|(p, c)| match c {
            'S' => Some(p),
            _ => None,
        })
        .unwrap();
    // let steps = find_first_steps(&tiles, &start);
    let steps = vec![
        Point { x: 0, y: 1 },
        Point { x: 0, y: -1 },
        Point { x: -1, y: 0 },
        Point { x: 1, y: 0 },
    ];
    let main_loop = steps
        .iter()
        .find_map(|s| {
            let mut current_tile = Tile {
                label: 'S',
                coords: start.clone(),
            };
            let mut next_step = Some(s.clone());
            let mut path = Vec::<Tile>::new();
            loop {
                match next_step {
                    Some(ns) => {
                        match step(&current_tile.coords, &ns, &tiles) {
                            Some(next_tile) => {
                                if next_tile.label == 'S' {
                                    return Some(path);
                                }
                                path.push(next_tile.clone());
                                next_step = Some(get_next_step(&ns, &next_tile));
                                current_tile = next_tile.clone();
                            }
                            None => {
                                return None;
                            }
                        };
                    }
                    None => {
                        return None;
                    }
                };
            }
        })
        .expect("No loop found");

    ((main_loop.len() + 1) / 2) as i32
}

fn parse_tiles(input: String) -> HashMap<Point, char> {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    Point {
                        x: x as i32,
                        y: y as i32,
                    },
                    c,
                )
            })
        })
        .flatten()
        .filter(|(_, c)| *c != '.')
        .collect::<HashMap<_, _>>()
}

fn step(p: &Point, direction: &Point, tiles: &HashMap<Point, char>) -> Option<Tile> {
    let next_p = Point {
        x: p.x + direction.x,
        y: p.y + direction.y,
    };
    let next_tile = tiles.get(&next_p);

    if next_tile.is_none() {
        return None;
    }

    let next_tile = next_tile.unwrap();
    let valid_tiles = match direction {
        Point { x: 0, y: -1 } => ['|', '7', 'F', 'S'],
        Point { x: 0, y: 1 } => ['|', 'L', 'J', 'S'],
        Point { x: 1, y: 0 } => ['-', '7', 'J', 'S'],
        Point { x: -1, y: 0 } => ['-', 'L', 'F', 'S'],
        _ => panic!("Invalid direction"),
    };

    if valid_tiles.contains(next_tile) {
        return Some(Tile {
            label: *next_tile,
            coords: next_p,
        });
    } else {
        return None;
    }
}

fn get_next_step(current_step: &Point, next_tile: &Tile) -> Point {
    match next_tile.label {
        '|' => match current_step {
            Point { x: 0, y: 1 } => Point { x: 0, y: 1 },
            _ => Point { x: 0, y: -1 },
        },
        '-' => match current_step {
            Point { x: 1, y: 0 } => Point { x: 1, y: 0 },
            _ => Point { x: -1, y: 0 },
        },
        '7' => match current_step {
            Point { x: 1, y: 0 } => Point { x: 0, y: 1 },
            _ => Point { x: -1, y: 0 },
        },
        'J' => match current_step {
            Point { x: 0, y: 1 } => Point { x: -1, y: 0 },
            _ => Point { x: 0, y: -1 },
        },
        'L' => match current_step {
            Point { x: 0, y: 1 } => Point { x: 1, y: 0 },
            _ => Point { x: 0, y: -1 },
        },
        'F' => match current_step {
            Point { x: -1, y: 0 } => Point { x: 0, y: 1 },
            _ => Point { x: 1, y: 0 },
        },
        _ => panic!("Invalid tile"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tiles() {
        let input = ".....
.S-7.
.|.|.
.L-J.
....."
            .to_string();
        let expected = HashMap::from_iter([
            (Point { x: 1, y: 1 }, 'S'),
            (Point { x: 2, y: 1 }, '-'),
            (Point { x: 3, y: 1 }, '7'),
            (Point { x: 1, y: 2 }, '|'),
            (Point { x: 3, y: 2 }, '|'),
            (Point { x: 1, y: 3 }, 'L'),
            (Point { x: 2, y: 3 }, '-'),
            (Point { x: 3, y: 3 }, 'J'),
        ]);
        let result = parse_tiles(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_step() {
        let input_p = Point { x: 1, y: 1 };
        let input_direction = Point { x: 1, y: 0 };
        let input_tiles = HashMap::from_iter([
            (Point { x: 1, y: 1 }, 'S'),
            (Point { x: 2, y: 1 }, '-'),
            (Point { x: 3, y: 1 }, '7'),
            (Point { x: 1, y: 2 }, '|'),
            (Point { x: 3, y: 2 }, '|'),
            (Point { x: 1, y: 3 }, 'L'),
            (Point { x: 2, y: 3 }, '-'),
            (Point { x: 3, y: 3 }, 'J'),
        ]);
        let expected = Some(Tile {
            label: '-',
            coords: Point { x: 2, y: 1 },
        });
        let result = step(&input_p, &input_direction, &input_tiles);
        assert_eq!(result, expected);
    }

    #[test]
    fn it_works() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
            .to_string();
        let expected = 4;
        let result = part1(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            .to_string();
        let expected = 8;
        let result = part1(input);
        assert_eq!(result, expected);
    }
}
