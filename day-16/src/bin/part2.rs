use std::collections::VecDeque;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn move_light(light: Light, row_count: usize, col_count: usize) -> Option<Light> {
    match light.direction {
        Direction::Up => Some(Light {
            row: light.row.checked_sub(1)?,
            ..light
        }),
        Direction::Right => Some(Light {
            col: {
                if light.col + 1 == col_count {
                    return None;
                } else {
                    light.col + 1
                }
            },
            ..light
        }),
        Direction::Down => Some(Light {
            row: {
                if light.row + 1 == row_count {
                    return None;
                } else {
                    light.row + 1
                }
            },
            ..light
        }),
        Direction::Left => Some(Light {
            col: light.col.checked_sub(1)?,
            ..light
        }),
    }
}

fn reflect_direction(direction: Direction, reflector: char) -> Box<dyn Iterator<Item = Direction>> {
    match (direction, reflector) {
        (Direction::Up, '|') | (Direction::Right, '/') | (Direction::Left, '\\') => {
            Box::new(std::iter::once(Direction::Up))
        }
        (Direction::Up, '/') | (Direction::Right, '-') | (Direction::Down, '\\') => {
            Box::new(std::iter::once(Direction::Right))
        }
        (Direction::Right, '\\') | (Direction::Down, '|') | (Direction::Left, '/') => {
            Box::new(std::iter::once(Direction::Down))
        }
        (Direction::Up, '\\') | (Direction::Down, '/') | (Direction::Left, '-') => {
            Box::new(std::iter::once(Direction::Left))
        }
        (Direction::Up, '-') | (Direction::Down, '-') => {
            Box::new(std::iter::once(Direction::Right).chain(std::iter::once(Direction::Left)))
        }
        (Direction::Right, '|') | (Direction::Left, '|') => {
            Box::new(std::iter::once(Direction::Up).chain(std::iter::once(Direction::Down)))
        }
        _ => unreachable!(),
    }
}

#[derive(Copy, Clone)]
struct Light {
    row: usize,
    col: usize,
    direction: Direction,
}

#[derive(PartialEq, Debug)]
struct ReflectorUsage {
    row: usize,
    col: usize,
    from_direction: Direction,
}

fn part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let row_count = grid.len();
    let col_count = grid[0].len();
    (0..col_count)
        .map(|j| try_light_at(grid.clone(), 0, j, Direction::Down))
        .chain(
            (0..row_count).map(|i| try_light_at(grid.clone(), i, col_count - 1, Direction::Left)),
        )
        .chain((0..col_count).map(|j| try_light_at(grid.clone(), row_count - 1, j, Direction::Up)))
        .chain((0..row_count).map(|i| try_light_at(grid.clone(), i, 0, Direction::Right)))
        .max()
        .unwrap()
}

fn try_light_at(
    mut grid: Vec<Vec<char>>,
    row: usize,
    col: usize,
    initial_direction: Direction,
) -> usize {
    let row_count = grid.len();
    let col_count = grid[0].len();
    let initial_directions = if grid[row][col] == '.' {
        grid[row][col] = '#';
        Box::new(std::iter::once(initial_direction))
    } else {
        reflect_direction(initial_direction, grid[row][col])
    };
    let mut queue = VecDeque::from(
        initial_directions
            .map(|initial_direction| Light {
                row,
                col,
                direction: initial_direction,
            })
            .collect::<Vec<Light>>(),
    );
    let mut used_reflectors: Vec<ReflectorUsage> = Vec::new();
    while !queue.is_empty() {
        let current_light = queue.pop_front().unwrap();
        let Some(next_light) = move_light(current_light, row_count, col_count) else {
            continue;
        };
        if matches!(grid[next_light.row][next_light.col], '#' | '.') {
            queue.push_back(next_light);
            grid[next_light.row][next_light.col] = '#';
        } else {
            let reflector_usage = ReflectorUsage {
                row: next_light.row,
                col: next_light.col,
                from_direction: next_light.direction,
            };
            if used_reflectors.contains(&reflector_usage) {
                continue;
            }
            used_reflectors.push(reflector_usage);
            for reflected_direction in
                reflect_direction(next_light.direction, grid[next_light.row][next_light.col])
            {
                queue.push_back(Light {
                    direction: reflected_direction,
                    ..next_light
                });
            }
        }
    }
    grid.into_iter()
        .enumerate()
        .map(|(i, line)| {
            line.into_iter()
                .enumerate()
                .filter(|&(j, c)| match c {
                    '#' => true,
                    '.' => false,
                    _ => used_reflectors
                        .iter()
                        .any(|used_reflector| used_reflector.row == i && used_reflector.col == j),
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn part2_test() {
        let result = part2(
r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        );
        assert_eq!(result, 51);
    }
}
