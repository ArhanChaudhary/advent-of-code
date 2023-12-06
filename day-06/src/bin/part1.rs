use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let input_numbers: Vec<u32> = Regex::new(r"\d+")
        .unwrap()
        .find_iter(input)
        .map(|number| number.as_str().parse::<u32>().unwrap())
        .collect();
    let mut time_distance_map: Vec<[i32; 2]> = Vec::new();
    for i in 0..input_numbers.len() / 2 {
        time_distance_map.push([
            input_numbers[i] as i32,
            input_numbers[i + input_numbers.len() / 2] as i32,
        ]);
    }
    time_distance_map
        .iter()
        .map(|[time, distance]| {
            let disc = ((time.pow(2) - 4 * distance) as f64).sqrt();
            assert!(disc >= 0.0);
            let x1 = (*time as f64 + disc) / 2.0;
            let x1 = if x1.fract() == 0.0 {
                x1 - 1.0
            } else {
                x1.floor()
            } as u32;
            let x2 = (*time as f64 - disc) / 2.0;
            let x2 = if x2.fract() == 0.0 {
                x2 + 1.0
            } else {
                x2.ceil()
            } as u32;
            x1 - x2 + 1
        })
        .fold(1, |sum, x| sum * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, 288);
    }
}
