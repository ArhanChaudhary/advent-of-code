fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn total_positions_recursion(
    starting_from: usize,
    space: String,
    mut groups: Vec<usize>,
) -> Vec<String> {
    if groups.len() == 0 {
        if space.chars().skip(starting_from).all(|c| c != '#') {
            return vec![space];
        } else {
            return Vec::new();
        }
    }
    let group = groups.pop().unwrap();
    space
        .chars()
        .skip(starting_from)
        .take_while(|&c| c != '#')
        .chain(
            space
                .chars()
                .skip(starting_from)
                .skip_while(|&c| c != '#')
                .take(group),
        )
        .collect::<Vec<char>>()
        .windows(group)
        .enumerate()
        .filter_map(|(i, window)| {
            if window.iter().any(|&c| c == '.') {
                return None;
            }
            let i = i + starting_from;
            let try_new_space: String = space
                .char_indices()
                .map(|(j, c)| {
                    if (0..i as isize - 1).contains(&(j as isize)) {
                        if c == '?' {
                            Some('.')
                        } else {
                            Some(c)
                        }
                    } else if j as isize == i as isize - 1 || j == i + group {
                        if c == '#' {
                            None
                        } else {
                            Some('.')
                        }
                    } else if (i..i + group).contains(&j) {
                        Some('#')
                    } else {
                        Some(c)
                    }
                })
                .collect::<Option<String>>()?;
            Some(total_positions_recursion(
                i + group + 1,
                try_new_space,
                groups.clone(),
            ))
        })
        .flatten()
        .collect()
}

fn total_positions(space: &str, mut groups: Vec<usize>) -> usize {
    groups.reverse();
    total_positions_recursion(0, space.to_string(), groups).len()
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            total_positions(
                split.next().unwrap(),
                split
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );
        assert_eq!(result, 21);
    }
}
