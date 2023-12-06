use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq)]
struct Range {
    start: i64,
    end: i64,
    shift: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    from: String,
    to: String,
    ranges: Vec<Range>,
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let output = part2(&input);
    dbg!(output);
}

fn part2(input: &str) -> i64 {
    let (seeds, alamac) = parse_almanac(input);

    let transformed = dbg!(seeds
        .into_iter()
        .map(|s| convert("seed".to_string(), s, &alamac))
        .next()
        .unwrap());

    transformed
        .iter()
        .map(|r| r.start)
        .min()
        .expect("Couldn't find min location")
}

fn convert(from: String, range: Range, alamac: &HashMap<String, Map>) -> Vec<Range> {
    if &from == "location" {
        return vec![range];
    }
    let map = alamac.get(&from).expect("Category not found");
    dbg!(&map.to);
    let mut transformations = map.ranges.iter().collect::<Vec<&Range>>();
    transformations.sort_by_key(|t| t.start);
    let mut padding = transformations
        .windows(2)
        .filter_map(|t| {
            if t[0].end > t[1].start {
                Some(Range {
                    start: t[0].end + 1,
                    end: t[1].start - 1,
                    shift: 0,
                })
            } else {
                None
            }
        })
        .collect::<Vec<Range>>();
    let first = transformations.first().unwrap();
    if first.start != 0 {
        padding.push(Range {
            start: 0,
            end: first.start - 1,
            shift: 0,
        });
    }
    padding.push(Range {
        start: transformations.last().unwrap().end + 1,
        end: i64::MAX,
        shift: 0,
    });
    let mut padding_ref = padding.iter().collect::<Vec<&Range>>();
    transformations.append(&mut padding_ref);

    transformations
        .iter()
        .filter_map(
            |t| match (t.start.cmp(&range.start), t.end.cmp(&range.end)) {
                (Ordering::Less | Ordering::Equal, Ordering::Greater | Ordering::Equal) => {
                    Some(Range {
                        start: range.start + t.shift,
                        end: range.end + t.shift,
                        shift: t.shift,
                    })
                }
                (Ordering::Greater, Ordering::Less) => Some(Range {
                    start: t.start + t.shift,
                    end: t.end + t.shift,
                    shift: t.shift,
                }),
                (Ordering::Less | Ordering::Equal, Ordering::Less) => Some(Range {
                    start: range.start + t.shift,
                    end: t.end + t.shift,
                    shift: t.shift,
                }),
                (Ordering::Greater, Ordering::Greater | Ordering::Equal) => Some(Range {
                    start: t.start + t.shift,
                    end: range.end + t.shift,
                    shift: t.shift,
                }),
            },
        )
        .filter(|r| r.start <= r.end)
        .map(|r| convert(map.to.clone(), dbg!(r), &alamac))
        .flatten()
        .collect::<Vec<Range>>()
}

fn parse_almanac(input: &str) -> (Vec<Range>, HashMap<String, Map>) {
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

fn parse_seeds(seeds_str: &str) -> Vec<Range> {
    seeds_str
        .strip_prefix("seeds:")
        .expect("Prefix not found")
        .split_whitespace()
        .map(|num| num.parse::<i64>().expect("Could not parse seed"))
        .collect::<Vec<i64>>()
        .chunks(2)
        .map(|pair| Range {
            start: pair[0],
            end: pair[0] + pair[1] - 1,
            shift: 0,
        })
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

    let range = lines.map(parse_range).collect::<Vec<Range>>();

    Map {
        from,
        to,
        ranges: range,
    }
}

fn parse_range(line: &str) -> Range {
    let mut columns = line.split_whitespace();
    let dst_start = columns
        .next()
        .expect("Could not find destination start")
        .parse::<i64>()
        .expect("Could not parse destination start");
    let src_start = columns
        .next()
        .expect("Could not find source start")
        .parse::<i64>()
        .expect("Could not parse source start");
    let len = columns
        .next()
        .expect("Could not find length")
        .parse::<i64>()
        .expect("Could not parse length");
    Range {
        start: src_start,
        end: src_start + len - 1,
        shift: dst_start - src_start,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seeds() {
        let input = "seeds: 79 14 55 13";
        let expected = vec![
            Range {
                start: 79,
                end: 92,
                shift: 0,
            },
            Range {
                start: 55,
                end: 67,
                shift: 0,
            },
        ];
        let result = parse_seeds(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_range() {
        let input = "50 98 2";
        let expected = Range {
            start: 98,
            end: 99,
            shift: -48,
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
            ranges: vec![
                Range {
                    start: 98,
                    end: 99,
                    shift: -48,
                },
                Range {
                    start: 7,
                    end: 10,
                    shift: 50,
                },
            ],
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
        let expected_seeds = vec![
            Range {
                start: 79,
                end: 92,
                shift: 0,
            },
            Range {
                start: 55,
                end: 67,
                shift: 0,
            },
        ];
        let expected_maps = HashMap::from_iter([
            (
                "seed".to_string(),
                Map {
                    from: "seed".to_string(),
                    to: "soil".to_string(),
                    ranges: vec![Range {
                        start: 98,
                        end: 99,
                        shift: -48,
                    }],
                },
            ),
            (
                "soil".to_string(),
                Map {
                    from: "soil".to_string(),
                    to: "fertilizer".to_string(),
                    ranges: vec![Range {
                        start: 52,
                        end: 53,
                        shift: -15,
                    }],
                },
            ),
        ]);
        let (result_seeds, result_maps) = parse_almanac(input);
        assert_eq!(result_seeds, expected_seeds);
        assert_eq!(result_maps, expected_maps);
    }

    #[test]
    fn it_works() {
        let input = "seeds: 79 14

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
        let expected = 47;
        let result = part2(input);
        assert_eq!(result, expected);
    }
}
