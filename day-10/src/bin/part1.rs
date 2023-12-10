fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

struct Pipe {
    index: usize,
    prev_index: usize,
}

impl Pipe {
    fn from_next_index(&self, index_offset: isize) -> Pipe {
        Pipe {
            index: (self.index as isize + index_offset) as usize,
            prev_index: self.index,
        }
    }
}

fn part1(input: &str) -> usize {
    let line_length: isize = input.lines().next().unwrap().len() as isize;
    let input: String = input.chars().filter(|c| *c != '\n').collect();
    let start_index = input
        .chars()
        .enumerate()
        .find_map(|(i, c)| if c == 'S' { Some(i) } else { None })
        .unwrap();
    let mut curr = Pipe {
        index: start_index,
        prev_index: start_index,
    };
    let mut distance = 0;
    loop {
        let curr_tile = input.chars().nth(curr.index).unwrap();
        if curr_tile == 'S' && curr.index != curr.prev_index {
            // floor division
            return distance / 2;
        }
        let next = match curr.index as isize - curr.prev_index as isize {
            0 => {
                assert_eq!(curr_tile, 'S');
                if matches!(
                    input
                        .chars()
                        .nth(curr.index - line_length as usize)
                        .unwrap(),
                    '|' | 'F' | '7'
                ) {
                    curr.from_next_index(-line_length)
                } else if matches!(input.chars().nth(curr.index + 1).unwrap(), '-' | 'J' | '7') {
                    curr.from_next_index(1)
                } else if matches!(
                    input
                        .chars()
                        .nth(curr.index + line_length as usize)
                        .unwrap(),
                    '|' | 'L' | 'J'
                ) {
                    curr.from_next_index(line_length)
                } else if matches!(input.chars().nth(curr.index - 1).unwrap(), '-' | 'L' | 'F') {
                    curr.from_next_index(-1)
                } else {
                    unreachable!();
                }
            }
            1 => {
                // came from left
                match curr_tile {
                    '-' => curr.from_next_index(1),
                    'J' => curr.from_next_index(-line_length),
                    '7' => curr.from_next_index(line_length),
                    _ => unreachable!(),
                }
            }
            diff if diff == line_length => {
                // came from up
                match curr_tile {
                    '|' => curr.from_next_index(line_length),
                    'L' => curr.from_next_index(1),
                    'J' => curr.from_next_index(-1),
                    _ => unreachable!(),
                }
            }
            -1 => {
                // came from right
                match curr_tile {
                    '-' => curr.from_next_index(-1),
                    'F' => curr.from_next_index(line_length),
                    'L' => curr.from_next_index(-line_length),
                    _ => unreachable!(),
                }
            }
            diff if diff == -line_length => {
                // came from down
                match curr_tile {
                    '|' => curr.from_next_index(-line_length),
                    'F' => curr.from_next_index(1),
                    '7' => curr.from_next_index(-1),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        };
        curr = next;
        distance += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        assert_eq!(result, 4);

        let result = part1(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        assert_eq!(result, 8)
    }
}
