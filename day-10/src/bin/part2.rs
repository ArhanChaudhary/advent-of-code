use itertools::Itertools;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> usize {
    let line_length: usize = input.lines().next().unwrap().len();
    let input_chars: Vec<char> = input.chars().filter(|c| *c != '\n').collect();
    let start_index = input_chars
        .iter()
        .enumerate()
        .find_map(|(i, c)| if *c == 'S' { Some(i) } else { None })
        .unwrap();
    let mut curr;
    let mut prev = start_index;
    let mut diff = 0;
    let mut loop_indexes: Vec<usize> = vec![start_index];
    loop {
        let curr_tile = input_chars[prev];
        if curr_tile == 'S' && diff != 0 {
            break;
        }
        curr = (prev as isize
            + match diff {
                0 => {
                    assert_eq!(curr_tile, 'S');
                    if matches!(&input_chars[prev + line_length], '|' | 'L' | 'J') {
                        line_length as isize
                    } else if matches!(&input_chars[prev - 1], '-' | 'L' | 'F') {
                        -1
                    } else if matches!(&input_chars[prev - line_length], '|' | 'F' | '7') {
                        -(line_length as isize)
                    } else if matches!(&input_chars[prev + 1], '-' | 'J' | '7') {
                        1
                    } else {
                        unreachable!();
                    }
                }
                1 => {
                    // came from left
                    match curr_tile {
                        '-' => 1,
                        'J' => -(line_length as isize),
                        '7' => line_length as isize,
                        _ => unreachable!(),
                    }
                }
                diff if diff == line_length as isize => {
                    // came from up
                    match curr_tile {
                        '|' => line_length as isize,
                        'L' => 1,
                        'J' => -1,
                        _ => unreachable!(),
                    }
                }
                -1 => {
                    // came from right
                    match curr_tile {
                        '-' => -1,
                        'F' => line_length as isize,
                        'L' => -(line_length as isize),
                        _ => unreachable!(),
                    }
                }
                diff if diff == -(line_length as isize) => {
                    // came from down
                    match curr_tile {
                        '|' => -(line_length as isize),
                        'F' => 1,
                        '7' => -1,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }) as usize;
        diff = curr as isize - prev as isize;
        loop_indexes.push(curr);
        prev = curr;
    }

    let expanded_line_length = line_length * 2 - 1;
    let to_expanded_index = |i: usize| {
        let row = i / line_length;
        let col = i % line_length;
        let new_row = row * 2;
        let new_col = col * 2;
        new_row * expanded_line_length + new_col
    };
    let loop_indexes: Vec<usize> = loop_indexes
        .into_iter()
        .map(to_expanded_index)
        .tuple_windows::<(usize, usize)>()
        .flat_map(|(prev, next)| [prev, (prev + next) / 2])
        .collect();
    let start_index = to_expanded_index(start_index);
    let line_length = expanded_line_length;
    let line_count = input.lines().count() * 2 - 1;
    let input_chars: Vec<char> = input
        .lines()
        .flat_map(|line| {
            let expanded_line: Box<dyn Iterator<Item = char>> =
                Box::new(line.chars().flat_map(|c| [c, '.']).take(line_length));
            let dot_line: Box<dyn Iterator<Item = char>> =
                Box::new(std::iter::repeat('.').take(line_length));
            [expanded_line, dot_line]
        })
        .take(line_count)
        .flatten()
        .collect();

    let flooded_input_chars = [
        start_index - 1 + line_length,
        start_index + 1 + line_length,
        start_index - 1 - line_length,
        start_index + 1 - line_length,
    ]
    .into_iter()
    .find_map(|try_index_in_loop| {
        let mut queue = VecDeque::from([try_index_in_loop]);
        let mut flooded_input_chars = input_chars.clone();
        while queue.len() != 0 {
            let i = queue.pop_front().unwrap() as isize;
            for new_i in [
                i - line_length as isize,
                i + 1,
                i + line_length as isize,
                i - 1,
            ] {
                if new_i < 0 || new_i as usize >= flooded_input_chars.len() {
                    return None;
                }
                let new_i = new_i as usize;
                if loop_indexes.contains(&new_i) || flooded_input_chars[new_i] == 'I' {
                    continue;
                }
                queue.push_back(new_i);
                flooded_input_chars[new_i] = 'I';
            }
        }
        Some(flooded_input_chars)
    })
    .unwrap();
    flooded_input_chars
        .chunks(line_length)
        .step_by(2)
        .map(|line| {
            line.iter()
                .copied()
                .step_by(2)
                .filter(|&c| c == 'I')
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test() {
        let result = part2(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        );
        assert_eq!(result, 4);

        let result = part2(
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        );
        assert_eq!(result, 8)
    }
}
