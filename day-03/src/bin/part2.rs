use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let input_lines: Vec<&str> = input.lines().collect();
    let all_number_to_gear_position_maps: Vec<(u32, (u32, u32))> = input_lines
        .iter()
        .enumerate()
        .filter_map(|(i, &line)| {
            let mut chars = line.chars().enumerate();
            let mut number_to_gear_position_maps = Vec::new();
            loop {
                let character_to_gear_position_maps = chars
                    .by_ref()
                    .skip_while(|(_, c)| !c.is_numeric())
                    .take_while(|(_, c)| c.is_numeric())
                    .map(|(j, c)| {
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                let i = i as i32 + dy;
                                if i < 0 {
                                    continue;
                                }
                                match input_lines.get(i as usize) {
                                    Some(&line_check) => {
                                        let j = j as i32 + dx;
                                        if j < 0 {
                                            continue;
                                        }
                                        match line_check.chars().nth(j as usize) {
                                            Some(c_check) => {
                                                if c_check == '*' {
                                                    return (c, Some((i as u32, j as u32)));
                                                }
                                            }
                                            None => (),
                                        }
                                    }
                                    None => (),
                                }
                            }
                        }
                        (c, None)
                    })
                    .collect::<Vec<(char, Option<(u32, u32)>)>>();
                if character_to_gear_position_maps.len() == 0 {
                    break;
                }
                if let Some(gear_position) = character_to_gear_position_maps
                    .iter()
                    .find(|(_, o)| !o.is_none())
                    .map(|(_, o)| o.unwrap())
                {
                    number_to_gear_position_maps.push((
                        character_to_gear_position_maps
                            .iter()
                            .map(|(c, _)| *c)
                            .collect::<String>()
                            .parse::<u32>()
                            .unwrap(),
                        gear_position,
                    ));
                }
            }
            if number_to_gear_position_maps.len() == 0 {
                None
            } else {
                Some(number_to_gear_position_maps)
            }
        })
        .flatten()
        .collect();
    let mut gear_position_to_numbers_map: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
    for (number, gear_position) in all_number_to_gear_position_maps {
        if let Some(shared_gear_numbers) = gear_position_to_numbers_map.get_mut(&gear_position) {
            // shared_gears[&valid_number_gear] = Vec::new();
            shared_gear_numbers.push(number);
        } else {
            gear_position_to_numbers_map.insert(gear_position, vec![number]);
        }
    }
    gear_position_to_numbers_map
        .iter()
        .filter_map(|(_, numbers)| {
            if numbers.len() == 1 {
                None
            } else {
                Some(numbers.iter().fold(1, |product, number| product * number))
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
