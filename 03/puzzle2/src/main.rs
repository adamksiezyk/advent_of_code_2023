use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::Add,
};

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

fn find_neighbour_serial_numbers(
    serial_numbers: &Vec<SerialNumber>,
    symbols_indices: &HashSet<Point>,
) -> HashMap<Point, Vec<u32>> {
    let mut res: HashMap<Point, Vec<u32>> = symbols_indices
        .iter()
        .map(|s| (Point { ..*s }, Vec::new()))
        .collect();
    for sn in serial_numbers {
        let lower_x = sn.coords.x.saturating_sub(1);
        let upper_x = sn.coords.x.add(sn.number.len()).add(1);
        let lower_y = sn.coords.y.saturating_sub(1);
        let upper_y = sn.coords.y.add(2);
        for x in lower_x..upper_x {
            for y in lower_y..upper_y {
                let p = Point { x, y };
                if symbols_indices.contains(&p) {
                    res.get_mut(&p).unwrap().push(sn.number.parse().unwrap());
                }
            }
        }
    }
    res
}

fn main() {
    let schematic = fs::read_to_string("./input.txt").unwrap();

    let mut serial_numbers: Vec<SerialNumber> = Vec::new();
    let mut gears_indices: HashSet<Point> = HashSet::new();
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
                // if gear - store it, parse stored number and reset buffer
                '*' => {
                    gears_indices.insert(Point { x, y });
                    if !buff.is_empty() {
                        serial_numbers.push(parse_serial_number(&buff, &coords));
                        buff.clear();
                    }
                }
                // if dot, parse stored number and reset buffer
                _ => {
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

    let res = find_neighbour_serial_numbers(&serial_numbers, &gears_indices);
    let valid_res: u32 = res
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v.get(0).unwrap() * v.get(1).unwrap())
        .sum();

    println!("{:?}", valid_res);
}
