use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
enum OriginalPosition {
    Wall,
    Positions(Vec<Position>),
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Position {
    row: isize,
    col: isize,
    remaining_distance: usize,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Position {
    fn offset(&self, direction: &Direction) -> (isize, isize) {
        match direction {
            Direction::Up => (self.row - 1, self.col),
            Direction::Right => (self.row, self.col + 1),
            Direction::Down => (self.row + 1, self.col),
            Direction::Left => (self.row, self.col - 1),
        }
    }

    fn move_direction(
        &self,
        grid: &mut [Vec<OriginalPosition>],
        direction: Direction,
    ) -> Option<Self> {
        let (offset_row, offset_col) = self.offset(&direction);
        let original_offset_row = offset_row.rem_euclid(grid.len() as isize) as usize;
        let original_offset_col = offset_col.rem_euclid(grid[0].len() as isize) as usize;
        let offset_positions = match grid[original_offset_row][original_offset_col] {
            OriginalPosition::Wall => return None,
            OriginalPosition::Positions(ref mut positions) => positions,
        };
        if offset_positions.iter().any(|last_offset_position| {
            last_offset_position.row == offset_row && last_offset_position.col == offset_col
        }) {
            None
        } else {
            let new_offset_position = Position {
                row: offset_row,
                col: offset_col,
                remaining_distance: self.remaining_distance.checked_sub(1)?,
            };
            offset_positions.push(new_offset_position);
            Some(new_offset_position)
        }
    }
}

#[derive(Debug, Clone)]
struct StepSolution {
    steps: usize,
    possible_pots: usize,
}

fn part2(grid: &str, step_input: usize, parabola_count: usize, cycle_valid_after: usize) -> usize {
    let parabola_xs: [usize; 3] = ((step_input - cycle_valid_after) % (parabola_count * 2)
        + cycle_valid_after..)
        .step_by(parabola_count * 2)
        .take(3)
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();
    let steps_to_do = parabola_xs[2];
    let mut start_position: Option<Position> = None;
    let mut grid: Vec<Vec<OriginalPosition>> = grid
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '#' => OriginalPosition::Wall,
                    '.' => OriginalPosition::Positions(Vec::new()),
                    'S' => {
                        start_position = Some(Position {
                            row: i as isize,
                            col: j as isize,
                            remaining_distance: steps_to_do,
                        });
                        OriginalPosition::Positions(vec![start_position.unwrap()])
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
                curr.move_direction(&mut grid, Direction::Up),
                curr.move_direction(&mut grid, Direction::Right),
                curr.move_direction(&mut grid, Direction::Down),
                curr.move_direction(&mut grid, Direction::Left),
            ]
            .into_iter()
            .flatten(),
        );
    }

    let mut parabola_points: Vec<StepSolution> = parabola_xs
        .into_iter()
        .map(|parabola_step_point| StepSolution {
            steps: parabola_step_point,
            possible_pots: 0,
        })
        .collect();

    grid.into_iter().for_each(|row| {
        row.into_iter()
            .filter_map(|original_position| {
                if let OriginalPosition::Positions(positions) = original_position {
                    Some(positions)
                } else {
                    None
                }
            })
            .for_each(|positions| {
                positions
                    .into_iter()
                    .filter(|position| {
                        (((position.row + position.col) % 2 == 0)
                            != ((start_position.row + start_position.col) % 2 == 0))
                            != (step_input % 2 == 0)
                    })
                    .for_each(|position| {
                        parabola_points
                            .iter_mut()
                            .filter(|step_solution| {
                                step_solution.steps >= steps_to_do - position.remaining_distance
                            })
                            .for_each(|step_solution| {
                                step_solution.possible_pots += 1;
                            });
                    });
            });
    });
    math::calculate_parabola(
        &parabola_points[0],
        &parabola_points[1],
        &parabola_points[2],
    )
    .f_of(step_input)
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input, 26501365, 131, 0);
    dbg!(output);
}

mod math {
    use super::StepSolution;

    pub struct Parabola {
        a: f64,
        b: f64,
        c: f64,
    }

    impl Parabola {
        pub fn f_of(&self, x: usize) -> usize {
            (self.a * (x.pow(2) as f64) + self.b * (x as f64) + self.c).round() as usize
        }
    }

    // https://stackoverflow.com/questions/717762/how-to-calculate-the-vertex-of-a-parabola-given-three-points
    pub fn calculate_parabola(p1: &StepSolution, p2: &StepSolution, p3: &StepSolution) -> Parabola {
        let x1 = p1.steps as f64;
        let y1 = p1.possible_pots as f64;
        let x2 = p2.steps as f64;
        let y2 = p2.possible_pots as f64;
        let x3 = p3.steps as f64;
        let y3 = p3.possible_pots as f64;
        let denom = (x1 - x2) * (x1 - x3) * (x2 - x3);
        let a = (x3 * (y2 - y1) + x2 * (y1 - y3) + x1 * (y3 - y2)) / denom;
        let b = (x3 * x3 * (y1 - y2) + x2 * x2 * (y3 - y1) + x1 * x1 * (y2 - y3)) / denom;
        let c = (x2 * x3 * (x2 - x3) * y1 + x3 * x1 * (x3 - x1) * y2 + x1 * x2 * (x1 - x2) * y3)
            / denom;
        Parabola { a, b, c }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn part2_test() {
        let grid = "\
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
...........";
        let curried_part2 = |step_input| part2(grid, step_input, 11, 40);
        assert_eq!(curried_part2(50), 1594);
        assert_eq!(curried_part2(100), 6536);
        assert_eq!(curried_part2(500), 167004);
        assert_eq!(curried_part2(1000), 668697);
        assert_eq!(curried_part2(5000), 16733044);
    }
}
