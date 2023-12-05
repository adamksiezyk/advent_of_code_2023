use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct Range {
    dst_start: u32,
    src_start: u32,
    len: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    from: String,
    to: String,
    range: HashMap<u32, u32>,
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let output = part1(&input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let (seeds, alamac) = parse_almanac(input);

    seeds
        .into_iter()
        .map(|s| convert("seed".to_string(), s, &alamac))
        .min()
        .expect("Could not find solution")
}

fn convert(from: String, id: u32, alamac: &HashMap<String, Map>) -> u32 {
    let map = alamac.get(&from).expect("Category not found");
    let new_id = *map.range.get(&id).unwrap_or(&id);

    match map.to.as_str() {
        "location" => new_id,
        new_category => convert(new_category.to_string(), new_id, alamac),
    }
}

fn parse_almanac(input: &str) -> (Vec<u32>, HashMap<String, Map>) {
    let mut lines = input.lines();
    let seeds = parse_seeds(lines.next().unwrap());
    let _ = lines.next();

    let mut buff = String::new();
    let mut almanac = HashMap::<String, Map>::new();
    for line in lines {
        match line {
            l if l.is_empty() => {
                let map = parse_map(&buff);
                almanac.insert(map.from.clone(), map);
                buff.clear();
            }
            l => buff.push_str(&format!("{l}\n")),
        }
    }
    if !buff.is_empty() {
        let map = parse_map(&buff);
        almanac.insert(map.from.clone(), map);
    }

    (seeds, almanac)
}

fn parse_seeds(seeds_str: &str) -> Vec<u32> {
    seeds_str
        .strip_prefix("seeds:")
        .expect("Prefix not found")
        .split_whitespace()
        .map(|num| num.parse::<u32>().expect("Could not parse seed"))
        .collect()
}

fn parse_map(map_str: &str) -> Map {
    let mut lines = map_str.lines();
    let mut header = lines
        .next()
        .unwrap()
        .strip_suffix(" map:")
        .unwrap()
        .split("-to-");
    let from = header.next().expect("Could not parse from").to_string();
    let to = header.next().expect("Could not parse to").to_string();

    let range = lines
        .map(parse_range)
        .map(|mr| {
            (0..mr.len)
                .map(|i| (mr.src_start + i, mr.dst_start + i))
                .collect::<Vec<(u32, u32)>>()
        })
        .flatten()
        .collect::<HashMap<u32, u32>>();

    Map { from, to, range }
}

fn parse_range(line: &str) -> Range {
    let mut columns = line.split_whitespace();
    let dst_start = columns
        .next()
        .expect("Could not find destination start")
        .parse::<u32>()
        .expect("Could not parse destination start");
    let src_start = columns
        .next()
        .expect("Could not find source start")
        .parse::<u32>()
        .expect("Could not parse source start");
    let len = columns
        .next()
        .expect("Could not find length")
        .parse::<u32>()
        .expect("Could not parse length");
    Range {
        dst_start,
        src_start,
        len,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seeds() {
        let input = "seeds: 79 14 55 13";
        let expected = vec![79, 14, 55, 13];
        let result = parse_seeds(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_range() {
        let input = "50 98 2";
        let expected = Range {
            dst_start: 50,
            src_start: 98,
            len: 2,
        };
        let result = parse_range(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_map() {
        let input = "seed-to-soil map:
50 98 2
57 7 4";
        let expected = Map {
            from: "seed".to_string(),
            to: "soil".to_string(),
            range: HashMap::from_iter([(98, 50), (99, 51), (7, 57), (8, 58), (9, 59), (10, 60)]),
        };
        let result = parse_map(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_almanac() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2

soil-to-fertilizer map:
37 52 2";
        let expected_seeds = vec![79, 14, 55, 13];
        let expected_maps = HashMap::from_iter([
            (
                "seed".to_string(),
                Map {
                    from: "seed".to_string(),
                    to: "soil".to_string(),
                    range: HashMap::from_iter([(98, 50), (99, 51)]),
                },
            ),
            (
                "soil".to_string(),
                Map {
                    from: "soil".to_string(),
                    to: "fertilizer".to_string(),
                    range: HashMap::from_iter([(52, 37), (53, 38)]),
                },
            ),
        ]);
        let (result_seeds, result_maps) = parse_almanac(input);
        assert_eq!(result_seeds, expected_seeds);
        assert_eq!(result_maps, expected_maps);
    }

    #[test]
    fn it_works() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let expected = 35;
        let result = part1(input);
        assert_eq!(result, expected);
    }
}
