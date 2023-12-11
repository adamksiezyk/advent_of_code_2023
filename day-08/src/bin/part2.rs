use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    label: String,
    left: String,
    right: String,
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let output = part1(input);
    dbg!(output);
}

fn part1(input: String) -> u64 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect_vec();

    let nodes = lines
        .skip(1)
        .map(parse_node)
        .map(|n| (n.label.clone(), n))
        .collect::<HashMap<_, _>>();

    let current_nodes = find_starting_nodes(nodes.values());
    current_nodes
        .iter()
        .map(|n| {
            let mut current_node = *n;
            let mut hops = 0;
            for inst in instructions.iter().cycle() {
                hops += 1;
                let next_node_label = match inst {
                    'L' => current_node.left.as_str(),
                    _ => current_node.right.as_str(),
                };
                if next_node_label.ends_with('Z') {
                    return hops;
                }
                current_node = nodes.get(next_node_label).unwrap();
            }
            0
        })
        .reduce(|ans, x| lcm(ans, x))
        .unwrap()
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn parse_node(line: &str) -> Node {
    let mut parts = line.split(" = ");
    let label = parts.next().expect("Could not parse label").to_string();

    parts
        .next()
        .expect("Could not parse child nodes")
        .strip_prefix('(')
        .unwrap()
        .strip_suffix(')')
        .unwrap()
        .split(", ")
        .collect_tuple()
        .map(|(left, right)| Node {
            label,
            left: left.to_string(),
            right: right.to_string(),
        })
        .expect("Could not parse Node")
}

fn find_starting_nodes<'a, I>(nodes: I) -> Vec<&'a Node>
where
    I: Iterator<Item = &'a Node>,
{
    nodes.filter(|l| l.label.ends_with('A')).collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_node() {
        let input = "AAA = (BBB, CCC)";
        let expected = Node {
            label: "AAA".to_string(),
            left: "BBB".to_string(),
            right: "CCC".to_string(),
        };
        let result = parse_node(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_starting_nodes() {
        let input = vec![
            Node {
                label: "11A".to_string(),
                left: "11B".to_string(),
                right: "XXX".to_string(),
            },
            Node {
                label: "11B".to_string(),
                left: "XXX".to_string(),
                right: "11Z".to_string(),
            },
            Node {
                label: "11Z".to_string(),
                left: "11B".to_string(),
                right: "XXX".to_string(),
            },
            Node {
                label: "22A".to_string(),
                left: "22B".to_string(),
                right: "XXX".to_string(),
            },
            Node {
                label: "22B".to_string(),
                left: "22C".to_string(),
                right: "22C".to_string(),
            },
            Node {
                label: "22C".to_string(),
                left: "22Z".to_string(),
                right: "22Z".to_string(),
            },
            Node {
                label: "22Z".to_string(),
                left: "22B".to_string(),
                right: "22B".to_string(),
            },
            Node {
                label: "XXX".to_string(),
                left: "XXX".to_string(),
                right: "XXX".to_string(),
            },
        ];
        let expected = vec![
            Node {
                label: "11A".to_string(),
                left: "11B".to_string(),
                right: "XXX".to_string(),
            },
            Node {
                label: "22A".to_string(),
                left: "22B".to_string(),
                right: "XXX".to_string(),
            },
        ];
        let result = find_starting_nodes(input.iter())
            .iter()
            .map(|n| Node {
                label: n.label.clone(),
                left: n.left.clone(),
                right: n.right.clone(),
            })
            .collect_vec();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_works() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            .to_string();
        let expected = 6;
        let result = part1(input);
        assert_eq!(result, expected);
    }
}
