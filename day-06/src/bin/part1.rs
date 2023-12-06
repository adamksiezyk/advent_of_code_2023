use std::iter::zip;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let output = part1(input);
    dbg!(output);
}

fn part1(input: String) -> i32 {
    let mut lines = input.lines();
    let time = parse_line(lines.next().expect("Time not found"));
    let distance = parse_line(lines.next().expect("Distance not found"));
    zip(time, distance)
        .into_iter()
        .map(|(t, d)| quadratic_root(1, -t, d))
        .map(|(b1, b2)| (b2.ceil() as i32) - 1 - (b1.floor() as i32))
        .product::<i32>()
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split(':')
        .skip(1)
        .next()
        .expect("Invalid line")
        .split_whitespace()
        .map(|s| s.parse().expect("Could not parse number"))
        .collect()
}

fn quadratic_root(a: i32, b: i32, c: i32) -> (f32, f32) {
    let d = (b.pow(2) - 4 * a * c) as f32;
    let a = a as f32;
    let b = b as f32;
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
        let expected = 288;
        let result = part1(input);
        assert_eq!(result, expected);
    }
}
