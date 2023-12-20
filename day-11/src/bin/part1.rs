use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let output = part1(input);
    dbg!(output);
}

fn part1(input: String) -> usize {
    let universe = expand_universe(input);
    let galaxies = parse_galaxies(universe);
    galaxies
        .iter()
        .tuple_combinations()
        .map(|(g1, g2)| distance(g1, g2))
        .sum()
}

fn expand_universe(input: String) -> String {
    let n_cols = input.lines().next().unwrap().len();
    let mut universe = input
        .lines()
        .map(|line| match line.chars().all(|c| c == '.') {
            true => vec![line, line],
            false => vec![line],
        })
        .flatten()
        .map(|line| format!("{line}\n"))
        .collect::<String>();

    let new_cols = (0..n_cols)
        .filter_map(|i| {
            let col = universe
                .lines()
                .map(|line| line.chars().nth(i).unwrap())
                .collect::<String>();
            match col.chars().all(|c| c == '.') {
                true => Some(i),
                false => None,
            }
        })
        .collect_vec();
    for (i, j) in new_cols.iter().enumerate() {
        universe = universe
            .lines()
            .map(|line| format!("{}.{}", &line[..i + j], &line[i + j..]))
            .join("\n");
    }
    universe
}

fn parse_galaxies(universe: String) -> Vec<Point> {
    universe
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some(Point { x, y: y.clone() }),
                _ => None,
            })
        })
        .flatten()
        .collect_vec()
}

fn distance(start: &Point, end: &Point) -> usize {
    end.x.abs_diff(start.x) + end.y.abs_diff(start.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_galaxies() {
        let input = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#......."
            .to_string();
        let expected = vec![
            Point { x: 4, y: 0 },
            Point { x: 9, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 8, y: 5 },
            Point { x: 1, y: 6 },
            Point { x: 12, y: 7 },
            Point { x: 9, y: 10 },
            Point { x: 0, y: 11 },
            Point { x: 5, y: 11 },
        ];
        let result = parse_galaxies(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_distance() {
        let cases = vec![
            (Point { x: 1, y: 6 }, Point { x: 5, y: 11 }, 9),
            (Point { x: 4, y: 0 }, Point { x: 9, y: 10 }, 15),
            (Point { x: 0, y: 2 }, Point { x: 12, y: 7 }, 17),
            (Point { x: 0, y: 11 }, Point { x: 5, y: 11 }, 5),
        ];
        for (x0, x1, expected) in cases {
            let result = distance(&x0, &x1);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_expand_universe() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            .to_string();
        let expected = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#......."
            .to_string();
        let result = expand_universe(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn it_works() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            .to_string();
        let expected = 374;
        let result = part1(input);
        assert_eq!(result, expected);
    }
}
