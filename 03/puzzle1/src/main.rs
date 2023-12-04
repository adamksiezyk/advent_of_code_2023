use std::{collections::HashSet, ops::Add, fs};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct SerialNumber {
    coords: Point,
    number: String,
}

fn parse_serial_number(serial_number: &str, coords: &Point) -> SerialNumber {
    SerialNumber {
        coords: Point { ..*coords },
        number: serial_number.to_string(),
    }
}

fn has_neighbour_symbol(sn: &SerialNumber, symbols_indices: &HashSet<Point>) -> bool {
    let lower_x = sn.coords.x.saturating_sub(1);
    let upper_x = sn.coords.x.add(sn.number.len()).add(1);
    let lower_y = sn.coords.y.saturating_sub(1);
    let upper_y = sn.coords.y.add(2);
    for x in lower_x..upper_x {
        for y in lower_y..upper_y {
            if symbols_indices.contains(&Point { x, y }) {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let schematic = fs::read_to_string("./input.txt").unwrap();

    let mut serial_numbers: Vec<SerialNumber> = Vec::new();
    let mut symbols_indices: HashSet<Point> = HashSet::new();
    for (y, line) in schematic.lines().enumerate() {
        let mut buff = String::new();
        let mut coords = Point { x: 0, y: 0 };
        for (x, character) in line.chars().enumerate() {
            match character {
                // if digit, push to buffer and store index of beginning
                c if c.is_digit(10) => {
                    if buff.is_empty() {
                        coords = Point { x, y };
                    }
                    buff.push(c);
                }
                // if dot, parse stored number and reset buffer
                '.' => {
                    if !buff.is_empty() {
                        serial_numbers.push(parse_serial_number(&buff, &coords));
                        buff.clear();
                    }
                }
                // else special character - store it, parse stored number and reset buffer
                _ => {
                    symbols_indices.insert(Point { x, y });
                    if !buff.is_empty() {
                        serial_numbers.push(parse_serial_number(&buff, &coords));
                        buff.clear();
                    }
                }
            }
        }
        // end of line, parse stored number
        if !buff.is_empty() {
            serial_numbers.push(parse_serial_number(&buff, &coords));
        }
    }

    let valid_serial_numbers: u32 = serial_numbers
        .iter()
        .filter(|sn| has_neighbour_symbol(sn, &symbols_indices))
        .map(|sn| sn.number.parse::<u32>().unwrap())
        .sum();

    println!("{:?}", valid_serial_numbers);
}
