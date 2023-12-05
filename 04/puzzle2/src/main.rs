use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn process_card(card_id: u32, cards: &HashMap<u32, u32>) -> u32 {
    let num_matches = cards.get(&card_id).expect("Should not overflow");
    (card_id + 1..card_id + num_matches + 1)
        .map(|new_card_id| process_card(new_card_id, cards))
        .sum::<u32>() + 1
}

fn main() {
    let input = read_to_string("./input.txt").expect("File not found");

    let cards = input
        .lines()
        .map(|line| {
            let mut it = line.split(':');

            let card_id = it
                .next()
                .expect("No header found")
                .split_whitespace()
                .skip(1)
                .next()
                .expect("No Card ID found")
                .parse::<u32>()
                .expect("Could not parse Card ID");

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

            (card_id, count as u32)
        })
        .collect::<HashMap<u32, u32>>();

    let res = cards.keys().map(|card_id| process_card(*card_id, &cards)).sum::<u32>();
    println!("{res}");
}
