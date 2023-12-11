use std::collections::HashMap;

use itertools::Itertools;

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

fn part1(input: String) -> u32 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars();

    let nodes = lines
        .skip(1)
        .map(parse_node)
        .map(|n| (n.label.clone(), n))
        .collect::<HashMap<_, _>>();

    let mut hops = 0;
    let mut current_node = nodes.get("AAA").unwrap();
    for inst in instructions.cycle() {
        hops += 1;
        let next_node_label = match inst {
            'L' => current_node.left.as_str(),
            _ => current_node.right.as_str(),
        };
        if next_node_label == "ZZZ" {
            break;
        }
        current_node = nodes.get(next_node_label).unwrap();
    }

    hops
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
    fn it_works() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            .to_string();
        let expected = 6;
        let result = part1(input);
        assert_eq!(result, expected);
    }
}
