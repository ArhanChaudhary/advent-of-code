fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        let mut first: Option<u32> = None;
        let mut second: Option<u32> = None;
        for c in line.chars() {
            if let Some(as_digit) = c.to_digit(10) {
                if first.is_none() {
                    first = Some(as_digit);
                }
                second = Some(as_digit);
            }
        }
        let line_result = first.unwrap() * 10 + second.unwrap();
        count += line_result;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet",
        );
        assert_eq!(result, 142);
    }
}
