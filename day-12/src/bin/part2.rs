use itertools::Itertools;
use memoize::memoize;
use rayon::prelude::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[memoize]
fn total_positions(space: String, groups: Vec<usize>) -> usize {
    let group = groups[groups.len() - 1];
    let groups = &groups[0..groups.len() - 1];
    let mut remaining_upper_bound: usize =
        space.len() - groups.iter().sum::<usize>() - groups.len();
    if let Some(first_operational_block) = space.find('#') {
        remaining_upper_bound = remaining_upper_bound.min(first_operational_block + group);
    }
    space[..remaining_upper_bound]
        .chars()
        .collect::<Vec<char>>()
        .windows(group)
        .enumerate()
        .filter_map(|(i, window)| {
            if window.iter().any(|&c| c == '.') {
                None
            } else if i + group == space.len() {
                if groups.len() == 0 {
                    Some(1)
                } else {
                    None
                }
            } else if (i >= 1 && space.chars().nth(i - 1).unwrap() == '#')
                || space.chars().nth(i + group).unwrap() == '#'
            {
                None
            } else {
                let try_new_space = &space[i + group + 1..];
                if groups.len() == 0 {
                    if try_new_space.chars().all(|c| c != '#') {
                        Some(1)
                    } else {
                        None
                    }
                } else {
                    Some(total_positions(try_new_space.to_owned(), groups.to_vec()))
                }
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .par_lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let space =
                Itertools::intersperse(std::iter::repeat(split.next().unwrap()).take(5), "?")
                    .collect::<String>();
            let groups = std::iter::repeat(
                split
                    .next()
                    .unwrap()
                    .rsplit(",")
                    .map(|c| c.parse::<usize>().unwrap()),
            )
            .take(5)
            .flatten()
            .collect::<Vec<usize>>();
            total_positions(space, groups)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test() {
        let result = part2(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );
        assert_eq!(result, 525152);
    }
}
