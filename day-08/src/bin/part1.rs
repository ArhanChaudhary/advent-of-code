use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    let mut input_lines = input.lines().filter(|line| !line.is_empty());
    let directions = input_lines.next().unwrap();
    let mut location_map: HashMap<String, [String; 2]> = HashMap::new();
    for line in input_lines {
        if let Some(captures) = Regex::new(r"(\w\w\w) = \((\w\w\w), (\w\w\w)\)")
            .unwrap()
            .captures(line)
        {
            location_map.insert(
                captures[1].to_string(),
                [captures[2].to_string(), captures[3].to_string()],
            );
        } else {
            unreachable!();
        }
    }
    directions
        .chars()
        .cycle()
        .fold_while((0, "AAA"), |(count, source), c| {
            if source == "ZZZ" {
                return Done((count, &source));
            }
            let dest = location_map.get(source).unwrap();
            if c == 'R' {
                Continue((count + 1, &dest[1]))
            } else if c == 'L' {
                Continue((count + 1, &dest[0]))
            } else {
                unreachable!();
            }
        })
        .into_inner()
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, 6);
    }
}
