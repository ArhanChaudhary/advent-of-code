fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;
            for subset in line
                .chars()
                .skip_while(|c| *c != ':')
                .skip(2)
                .collect::<String>()
                .split("; ")
            {
                for cube_info in subset.split(", ") {
                    let cube_info: Vec<&str> = cube_info.split(" ").collect();
                    let cube_type = cube_info[1];
                    let cube_count = cube_info[0].parse::<u32>().unwrap();
                    if cube_type == "red" && cube_count > max_red {
                        max_red = cube_count;
                    }
                    if cube_type == "green" && cube_count > max_green {
                        max_green = cube_count;
                    }
                    if cube_type == "blue" && cube_count > max_blue {
                        max_blue = cube_count;
                    }
                }
            }
            max_red * max_green * max_blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286);
    }
}
