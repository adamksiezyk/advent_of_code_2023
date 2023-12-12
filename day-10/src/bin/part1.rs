use std::collections::HashMap;

use itertools::Itertools;

enum TileType {
    Start,
    Pipe,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let output = part1(input);
    dbg!(output);
}

fn part1(input: String) -> i32 {
    let tiles = parse_tiles(input);
    0
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

fn find_first_steps(tiles: &HashMap<Point, char>) -> Vec<Point> {
    let start = tiles
        .iter()
        .find_map(|(p, c)| match c {
            'S' => Some(p),
            _ => None,
        })
        .unwrap();
    vec![(0, 1), (0, -1), (-1, 0), (1, 0)]
        .iter()
        .map(|(dx, dy)| Point {
            x: start.x + dx,
            y: start.y + dy,
        })
        .filter_map(|p| tiles.contains_key(&p).then_some(p))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_steps() {
        let input = HashMap::from_iter([
            (Point { x: 1, y: 1 }, 'S'),
            (Point { x: 2, y: 1 }, '-'),
            (Point { x: 3, y: 1 }, '7'),
            (Point { x: 1, y: 2 }, '|'),
            (Point { x: 3, y: 2 }, '|'),
            (Point { x: 1, y: 3 }, 'L'),
            (Point { x: 2, y: 3 }, '-'),
            (Point { x: 3, y: 3 }, 'J'),
        ]);
        let expected = vec![Point { x: 1, y: 2 }, Point { x: 2, y: 1 }];
        let result = find_first_steps(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_start_node() {
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
    fn it_works() {
        let input = ".....
.S-7.
.|.|.
.L-J.
....."
            .to_string();
        let expected = 4;
        let result = part1(input);
        assert_eq!(result, expected);
    }
}
