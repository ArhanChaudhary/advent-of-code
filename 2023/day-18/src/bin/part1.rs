use itertools::Itertools;

#[derive(Clone, Copy)]
struct Point(isize, isize);

fn part1(input: &str) -> usize {
    let mut current_point = Point(0, 0);
    let mut border_displacement = 0;
    let mut boundary: Vec<Point> = Vec::new();
    for (line_direction, line_displacement) in input.lines().map(|line| {
        let mut split = line.split_whitespace();
        (
            split.next().unwrap().chars().exactly_one().unwrap(),
            split.next().unwrap().parse::<usize>().unwrap(),
        )
    }) {
        border_displacement += line_displacement;
        match line_direction {
            'U' => {
                current_point.1 += line_displacement as isize;
            }
            'R' => {
                current_point.0 += line_displacement as isize;
            }
            'D' => {
                current_point.1 -= line_displacement as isize;
            }
            'L' => {
                current_point.0 -= line_displacement as isize;
            }
            _ => unreachable!(),
        }
        boundary.push(current_point);
    }
    let area = (0..boundary.len())
        .map(|i| {
            let curr_y = boundary[i].1;
            let prev_x = if i == 0 {
                boundary[boundary.len() - 1]
            } else {
                boundary[i - 1]
            }
            .0;
            let next_x = if i == boundary.len() - 1 {
                boundary[0]
            } else {
                boundary[i + 1]
            }
            .0;
            curr_y * (prev_x - next_x)
        })
        .sum::<isize>().unsigned_abs()
        / 2;
    // A = i + b/2 - 1
    // i = A - b/2 + 1
    let interior_points = area - border_displacement / 2 + 1;
    interior_points + border_displacement
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
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
        );
        assert_eq!(result, 62);
    }
}
