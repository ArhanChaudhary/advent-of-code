use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let input_lines: Vec<&str> = input.lines().collect();
    let mut gear_position_to_numbers_map: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
    for (i, &line) in input_lines.iter().enumerate() {
        let mut chars = line.chars().enumerate();
        loop {
            let character_to_gear_position_maps = chars
                .by_ref()
                .skip_while(|(_, c)| !c.is_numeric())
                .take_while(|(_, c)| c.is_numeric())
                .map(|(j, c)| {
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let i = i as i32 + dy;
                            let j = j as i32 + dx;
                            if i >= 0
                                && j >= 0
                                && i < input_lines.len() as i32
                                && j < input_lines[0].len() as i32
                                && input_lines[i as usize].chars().nth(j as usize).unwrap() == '*'
                            {
                                return (c, Some((i as u32, j as u32)));
                            }
                        }
                    }
                    (c, None)
                })
                .collect::<Vec<(char, Option<(u32, u32)>)>>();
            if character_to_gear_position_maps.is_empty() {
                break;
            }
            if let Some(gear_position) = character_to_gear_position_maps
                .iter()
                .find(|(_, o)| !o.is_none())
                .map(|(_, o)| o.unwrap())
            {
                let number = character_to_gear_position_maps
                    .iter()
                    .map(|(c, _)| *c)
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
                if let Some(shared_gear_numbers) =
                    gear_position_to_numbers_map.get_mut(&gear_position)
                {
                    // shared_gears[&valid_number_gear] = Vec::new();
                    shared_gear_numbers.push(number);
                } else {
                    gear_position_to_numbers_map.insert(gear_position, vec![number]);
                }
            }
        }
    }
    gear_position_to_numbers_map
        .iter()
        .filter_map(|(_, numbers)| {
            if numbers.len() == 1 {
                None
            } else {
                Some(numbers.iter().product::<u32>())
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test() {
        let result = part2(
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
        assert_eq!(result, 467835);
    }
}
