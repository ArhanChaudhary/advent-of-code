use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
enum PositionStatus {
    Wall,
    Free,
    RemainingDistance(usize),
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
struct Position(usize, usize);
impl Position {
    fn move_offset(&self, grid: &mut [Vec<PositionStatus>], direction: Direction) -> Option<Self> {
        let (row_offset, col_offset) = match direction {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        };
        let new_row = self.0.checked_add_signed(row_offset)?;
        let new_col = self.1.checked_add_signed(col_offset)?;
        if new_row >= grid.len()
            || new_col >= grid[0].len()
            || grid[new_row][new_col] == PositionStatus::Wall
        {
            None
        } else if let PositionStatus::RemainingDistance(current_distance) = grid[self.0][self.1] {
            if matches!(grid[new_row][new_col], PositionStatus::RemainingDistance(_)) {
                None
            } else {
                grid[new_row][new_col] =
                    PositionStatus::RemainingDistance(current_distance.checked_sub(1)?);
                Some(Position(new_row, new_col))
            }
        } else {
            unreachable!();
        }
    }
}

const STEPS: usize = 64;

fn part1(input: &str) -> usize {
    let mut start_position: Option<Position> = None;
    let mut grid: Vec<Vec<PositionStatus>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '#' => PositionStatus::Wall,
                    '.' => PositionStatus::Free,
                    'S' => {
                        start_position = Some(Position(i, j));
                        PositionStatus::RemainingDistance(STEPS)
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    let start_position = start_position.expect("No start position found");
    let mut position_queue = VecDeque::from([start_position]);
    loop {
        let Some(curr) = position_queue.pop_front() else {
            break;
        };
        position_queue.extend(
            [
                curr.move_offset(&mut grid, Direction::Up),
                curr.move_offset(&mut grid, Direction::Right),
                curr.move_offset(&mut grid, Direction::Down),
                curr.move_offset(&mut grid, Direction::Left),
            ]
            .into_iter()
            .flatten(),
        );
    }
    let start_parity = (start_position.0 + start_position.1) % 2 == 0;
    // for i in grid {
    //     for j in i {
    //         let to_print = match j {
    //             PositionStatus::Free => ".".to_string(),
    //             PositionStatus::Wall => "#".to_string(),
    //             PositionStatus::RemainingDistance(d) => d.to_string(),
    //         };
    //         print!("{}", to_print);
    //         std::io::stdout().flush().unwrap();
    //     }
    //     println!();
    // }
    grid.into_iter()
        .enumerate()
        .map(|(i, row)| {
            row.into_iter()
                .enumerate()
                .filter(|(j, position)| {
                    if ((j + i) % 2 == 0) != start_parity {
                        false
                    } else {
                        matches!(position, PositionStatus::RemainingDistance(_))
                    }
                })
                .count()
        })
        .sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn part1_test() {
        let result = part1("\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
        );
        assert_eq!(result, 16);
    }
}
