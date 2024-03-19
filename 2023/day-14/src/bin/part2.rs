fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn spin_cycle(grid: &mut Grid) {
    let row_count = grid.len();
    let col_count = grid[0].len();
    for j in 0..col_count {
        for i in 0..row_count {
            if grid[i][j] != 'O' {
                continue;
            }
            let mut k = i;
            while k as isize > 0 && grid[k - 1][j] == '.' {
                grid[k - 1][j] = 'O';
                grid[k][j] = '.';
                k -= 1;
            }
        }
    }

    for row in grid.iter_mut().take(row_count) {
        for j in 0..col_count {
            if row[j] != 'O' {
                continue;
            }
            let mut k = j;
            while k as isize > 0 && row[k - 1] == '.' {
                row[k - 1] = 'O';
                row[k] = '.';
                k -= 1;
            }
        }
    }

    for j in 0..col_count {
        for i in (0..row_count).rev() {
            if grid[i][j] != 'O' {
                continue;
            }
            let mut k = i;
            while k + 1 < row_count && grid[k + 1][j] == '.' {
                grid[k + 1][j] = 'O';
                grid[k][j] = '.';
                k += 1;
            }
        }
    }

    for row in grid.iter_mut().take(row_count) {
        for j in (0..col_count).rev() {
            if row[j] != 'O' {
                continue;
            }
            let mut k = j;
            while k + 1 < col_count && row[k + 1] == '.' {
                row[k + 1] = 'O';
                row[k] = '.';
                k += 1;
            }
        }
    }
}

type Grid = Vec<Vec<char>>;
fn part1(input: &str) -> usize {
    let mut grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let mut seen_grids: Vec<Grid> = Vec::new();
    loop {
        if let Some(duplicate_grid_index) =
            seen_grids.iter().position(|seen_grid| seen_grid == &grid)
        {
            let final_grid = &seen_grids[duplicate_grid_index
                + (1000000000 - duplicate_grid_index) % (seen_grids.len() - duplicate_grid_index)];
            let row_count = final_grid.len();
            break final_grid
                .iter()
                .enumerate()
                .map(|(i, line)| (row_count - i) * line.iter().filter(|&&c| c == 'O').count())
                .sum();
        }
        seen_grids.push(grid.clone());
        spin_cycle(&mut grid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn part1_test() {
        let result = part1("\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );
        assert_eq!(result, 64);
    }
}
