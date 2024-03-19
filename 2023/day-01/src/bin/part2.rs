fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

const NUMBERS_STR: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = parse_digits(line);

            let first = digits.next().unwrap();
            let second = digits.last().unwrap_or(first);
            first * 10 + second
        })
        .sum()
}

fn parse_digits(line: &str) -> impl Iterator<Item = u32> + '_ {
    line.chars().enumerate().filter_map(|(i, c)| {
        if let Some(as_digit) = c.to_digit(10) {
            Some(as_digit)
        } else {
            let remaining = line.chars().skip(i).collect::<String>();
            NUMBERS_STR
                .into_iter()
                .find(|(number_str, _)| remaining.starts_with(number_str))
                .map(|(_, number)| number)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }
}
