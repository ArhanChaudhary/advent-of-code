fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> usize {
    let time_distance_map: Vec<usize> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .collect();
    let time = time_distance_map[0];
    let distance = time_distance_map[1];
    let disc = ((time.pow(2) - 4 * distance) as f64).sqrt();
    assert!(disc >= 0.0);
    let x1 = (time as f64 + disc) / 2.0;
    let x1 = if x1.fract() == 0.0 {
        x1 - 1.0
    } else {
        x1.floor()
    } as usize;
    let x2 = (time as f64 - disc) / 2.0;
    let x2 = if x2.fract() == 0.0 {
        x2 + 1.0
    } else {
        x2.ceil()
    } as usize;
    x1 - x2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test() {
        let result = part2(
            "Time:      71530
Distance:  940200",
        );
        assert_eq!(result, 71503);
    }
}
