use std::fs::read_to_string;

fn transform(line: &str) -> u32 {
    let first = line
        .chars()
        .filter(|c| c.is_digit(10))
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap();
    let second = line
        .chars()
        .rev()
        .filter(|c| c.is_digit(10))
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap();
    return first * 10 + second;
}

fn main() {
    let file = read_to_string("./input.txt").unwrap();
    let result: u32 = file.lines().map(transform).sum();

    println!("{}", result);
}
