fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let row_count = grid.len();
    let col_count = grid[0].len();
    for j in 0..col_count {
        for i in 0..row_count {
            if grid[i][j] != 'O' {
                continue;
            }
            let mut k = i;
            while k >= 1 && grid[k - 1][j] == '.' {
                grid[k - 1][j] = 'O';
                grid[k][j] = '.';
                k -= 1;
            }
        }
    }
    grid.into_iter()
        .enumerate()
        .map(|(i, line)| (row_count - i) * line.into_iter().filter(|&c| c == 'O').count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(
            "O....#....
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
        assert_eq!(result, 136);
    }
}
