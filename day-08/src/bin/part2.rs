use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn gcd(a: usize, b: usize) -> usize {
    let mut max = a;
    let mut min = b;
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

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn lcmm(nums: Vec<usize>) -> usize {
    nums.into_iter().reduce(|acc, a| lcm(acc, a)).unwrap()
}

fn part2(input: &str) -> usize {
    let mut input_lines = input.lines().filter(|&line| !line.is_empty());
    let directions = input_lines.next().unwrap();
    let mut unfound_cycles: Vec<String> = Vec::new();
    let mut location_map: HashMap<String, [String; 2]> = HashMap::new();
    for line in input_lines {
        let captures = Regex::new(r"(\w\w\w) = \((\w\w\w), (\w\w\w)\)")
            .unwrap()
            .captures(line)
            .unwrap();
        if captures[1].chars().last().unwrap() == 'A' {
            unfound_cycles.push(captures[1].to_string());
        }
        location_map.insert(
            captures[1].to_string(),
            [captures[2].to_string(), captures[3].to_string()],
        );
    }
    let mut directions = directions.chars().cycle();
    let mut cycles_counts: Vec<usize> = Vec::new();
    let mut cycle_counter = 0;
    while unfound_cycles.len() != 0 {
        let direction = directions.next().unwrap();
        unfound_cycles.retain_mut(|cycle| {
            if cycle.chars().last().unwrap() == 'Z' {
                cycles_counts.push(cycle_counter);
                return false;
            }
            let i = if direction == 'R' {
                1
            } else if direction == 'L' {
                0
            } else {
                unreachable!();
            };
            *cycle = location_map.get(cycle).unwrap()[i].clone();
            true
        });
        cycle_counter += 1;
    }
    lcmm(cycles_counts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test() {
        let result = part2(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, 6);
    }
}
