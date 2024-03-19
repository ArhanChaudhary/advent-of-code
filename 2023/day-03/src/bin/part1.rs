fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let input_lines: Vec<&str> = input.lines().collect();
    input_lines
        .iter()
        .enumerate()
        .map(|(i, &line)| {
            let mut chars = line.chars().enumerate();
            let mut line_sum = 0;
            loop {
                let number_character_map: Vec<(char, bool)> = chars
                    .by_ref()
                    .skip_while(|(_, c)| !c.is_numeric())
                    .take_while(|(_, c)| c.is_numeric())
                    .map(|(j, c)| {
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                if i as i32 + dy < 0 {
                                    continue;
                                }

                                if let Some(&line_check) = input_lines.get((i as i32 + dy) as usize) {
                                    if j as i32 + dx < 0 {
                                        continue;
                                    }
                                    if let Some(c_check) = line_check.chars().nth((j as i32 + dx) as usize) {
                                        if !c_check.is_numeric() && c_check != '.' {
                                            return (c, true);
                                        }
                                    }
                                }
                            }
                        }
                        (c, false)
                    })
                    .collect();
                if number_character_map.is_empty() {
                    break;
                }
                if number_character_map.iter().any(|(_, b)| *b) {
                    line_sum += number_character_map.iter().map(|(c, _)| *c).collect::<String>().parse::<u32>().unwrap();
                }
            }
            line_sum
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 4361);
    }
}
