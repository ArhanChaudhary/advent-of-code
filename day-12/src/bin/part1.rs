fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn total_positions(space: &str, groups: &[usize]) -> usize {
    let group = groups[groups.len() - 1];
    let groups = &groups[0..groups.len() - 1];
    space
        .chars()
        .take_while(|&c| c != '#')
        .chain(space.chars().skip_while(|&c| c != '#').take(group))
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
                    Some(total_positions(try_new_space, groups))
                }
            }
        })
        .sum()
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
                    .rsplit(",")
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
                    .as_slice(),
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
