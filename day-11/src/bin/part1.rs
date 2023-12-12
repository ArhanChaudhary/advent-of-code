use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

fn part1(input: &str) -> usize {
    let input_rows: Vec<&str> = input.lines().collect();
    let empty_cols: Vec<usize> = (0..input_rows[0].len())
        .filter(|&j| (0..input_rows.len()).all(|i| input_rows[i].chars().nth(j).unwrap() != '#'))
        .collect();
    let mut empty_row_count = 0;
    input_rows
        .iter()
        .enumerate()
        .filter_map(|(i, &row)| {
            let mut row_galaxies = row
                .chars()
                .enumerate()
                .filter_map(|(j, c)| {
                    if c == '#' {
                        Some(Position {
                            row: i + empty_row_count,
                            col: j + empty_cols
                                .iter()
                                .take_while(|&&empty_col| empty_col <= j)
                                .count(),
                        })
                    } else {
                        None
                    }
                })
                .peekable();
            if row_galaxies.peek().is_some() {
                Some(row_galaxies.collect::<Vec<Position>>())
            } else {
                empty_row_count += 1;
                None
            }
        })
        .flatten()
        .combinations(2)
        .map(|combination| {
            let g1 = combination[0];
            let g2 = combination[1];
            ((g1.row as isize - g2.row as isize).abs() + (g1.col as isize - g2.col as isize).abs())
                as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(result, 374);
    }
}
