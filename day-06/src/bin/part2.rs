fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let output = part1(input);
    dbg!(output);
}

fn part1(input: String) -> i64 {
    let mut lines = input.lines();
    let time = parse_line(lines.next().expect("Time not found"));
    let distance = parse_line(lines.next().expect("Distance not found"));
    let (b1, b2) = quadratic_root(1, -time, distance);
    (b2.ceil() as i64) - 1 - (b1.floor() as i64)
}

fn parse_line(line: &str) -> i64 {
    line.split(':')
        .skip(1)
        .next()
        .expect("Invalid line")
        .split_whitespace()
        .collect::<String>()
        .parse()
        .expect("Could not parse")
}

fn quadratic_root(a: i64, b: i64, c: i64) -> (f64, f64) {
    let d = (b.pow(2) - 4 * a * c) as f64;
    let a = a as f64;
    let b = b as f64;
    ((-b - d.sqrt()) / (2.0 * a), (-b + d.sqrt()) / (2.0 * a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Time:      7  15   30
Distance:  9  40  200"
            .to_string();
        let expected = 71503;
        let result = part1(input);
        assert_eq!(result, expected);
    }
}
