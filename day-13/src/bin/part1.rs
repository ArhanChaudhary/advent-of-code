use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn find_reflection_index(pattern: &Vec<String>) -> Option<usize> {
    pattern
        .iter()
        .tuple_windows()
        .enumerate()
        .find_map(|(i, (curr, next))| {
            if curr != next {
                return None;
            }
            let mut check_line_index = 0;
            return loop {
                let before = i as isize - check_line_index as isize;
                let after = i + check_line_index + 1;
                if before == -1 || after == pattern.len() {
                    break Some(i + 1);
                } else if pattern[before as usize] != pattern[after] {
                    break None;
                }
                check_line_index += 1;
            }
        })
}

fn rotate_pattern(pattern: &Vec<String>) -> Vec<String> {
    // literally rotate the entire thing by -90 degrees
    (0..pattern[0].len())
        .map(|j| {
            (0..pattern.len())
                .rev()
                .map(|i| pattern[i].chars().nth(j).unwrap())
                .collect()
        })
        .collect()
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| {
            let pattern: Vec<String> = pattern.lines().map(String::from).collect();
            if let Some(horizontal_reflection_index) = find_reflection_index(&pattern) {
                horizontal_reflection_index * 100
            } else {
                find_reflection_index(&rotate_pattern(&pattern)).unwrap()
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn part1_test() {
        let result = part1("\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(result, 405);
    }
}
