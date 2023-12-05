use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("./input.txt").expect("File not found");

    let result = input
        .lines()
        .map(|line| {
            let mut it = line.split(':');
            let _ = it.next();
            let body = it.next().expect("No body found");
            let mut it = body.split('|');
            let winning_numbers = it
                .next()
                .expect("No winning numbers found")
                .split_whitespace()
                .map(|num| num.parse().expect("Should be number"))
                .collect::<HashSet<u32>>();
            let my_numbers = it
                .next()
                .expect("No winning numbers found")
                .split_whitespace()
                .map(|num| num.parse().expect("Should be number"))
                .collect::<HashSet<u32>>();

            let count = my_numbers
                .iter()
                .filter(|num| winning_numbers.contains(num))
                .count();
            
            match count {
                0 => 0,
                c => 1 << (c - 1),
            }
        })
        .sum::<u32>();
    println!("{result}");
}
