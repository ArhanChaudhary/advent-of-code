fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn interpolate_prev(points: Vec<isize>) -> f64 {
    let at = -1.0;
    (0..points.len())
        .map(|i| {
            (0..points.len())
                .map(|j| {
                    if i == j {
                        1.0
                    } else {
                        (at - j as f64) / (i as f64 - j as f64)
                    }
                })
                .fold(points[i] as f64, |product, v| product * v)
        })
        .sum()
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            interpolate_prev(
                line.split_whitespace()
                    .map(|c| c.parse::<isize>().unwrap())
                    .collect(),
            )
        })
        .sum::<f64>()
        .round() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, 2);
    }
}
