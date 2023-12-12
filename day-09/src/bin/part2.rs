use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let output = part1(input);
    dbg!(output);
}

fn part1(input: String) -> i32 {
    input
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.parse::<i32>().unwrap()))
        .map(|seq| seq.collect_vec())
        .map(|seq| extrapolate(&seq))
        .sum()
}

fn diff(seq: &Vec<i32>) -> Vec<i32> {
    seq.windows(2).map(|s| s[1] - s[0]).collect_vec()
}

fn extrapolate(seq: &Vec<i32>) -> i32 {
    let mut buff = vec![*seq.first().unwrap()];
    let mut last_diff = seq.clone();
    loop {
        last_diff = diff(&last_diff);
        buff.push(*last_diff.first().unwrap());
        if last_diff.iter().all(|x| *x == 0) {
            break;
        }
    }
    buff.into_iter().rev().reduce(|ans, x| x - ans).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff() {
        let input = vec![0, 3, 6, 9, 12, 15];
        let expected = vec![3, 3, 3, 3, 3];
        let result = diff(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extrapolate() {
        let input = vec![10, 13, 16, 21, 30, 45];
        let expected = 5;
        let result = extrapolate(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn it_works() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            .to_string();
        let expected = 2;
        let result = part1(input);
        assert_eq!(result, expected);
    }
}
