fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Range {
        Range { start, end }
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }

    fn contains(&self, n: usize) -> bool {
        self.start <= n && n < self.end
    }

    fn get_nth(&self, n: usize) -> usize {
        n - self.start
    }

    fn nth(&self, n: usize) -> usize {
        self.start + n
    }
}

fn part1(input: &str) -> usize {
    let mut input_lines = input.lines();
    let seeds: Vec<usize> = input_lines
        .next()
        .unwrap()
        .chars()
        .skip_while(|c| *c != ':')
        .skip(1)
        .collect::<String>()
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect();
    let _ = input_lines.by_ref().next();
    let mut mappings: Vec<Vec<[Range; 2]>> = Vec::new();
    loop {
        let _ = input_lines.by_ref().next();
        let mut mapping: Vec<[Range; 2]> = input_lines
            .by_ref()
            .take_while(|&line| line != "")
            .map(|line| {
                let line_numbers: Vec<usize> = line
                    .split_whitespace()
                    .map(|number| number.parse::<usize>().unwrap())
                    .collect();
                [
                    Range::new(line_numbers[1], line_numbers[1] + line_numbers[2]),
                    Range::new(line_numbers[0], line_numbers[0] + line_numbers[2]),
                ]
            })
            .collect();
        if mapping.len() == 0 {
            break;
        }
        mapping.sort_by(|r, r2| r[1].start().cmp(&r2[1].start()));
        let mut prev_end = 0;
        let mut i = 0;
        while i != mapping.len() {
            let dest = mapping[i][1];
            let curr_start = dest.start();
            if prev_end != curr_start {
                mapping.insert(
                    i,
                    [
                        Range::new(prev_end, curr_start),
                        Range::new(prev_end, curr_start),
                    ],
                );
                i += 1;
            }
            prev_end = dest.end();
            i += 1;
        }
        mapping.push([
            Range::new(prev_end, usize::max_value()),
            Range::new(prev_end, usize::max_value()),
        ]);
        mapping.sort_by(|r, r2| r[0].start().cmp(&r2[0].start()));
        mappings.push(mapping);
    }
    dbg!(mappings.clone());
    seeds
        .iter()
        .map(|&seed| {
            mappings.iter().fold(seed, |source, mapping| {
                let source_map = mapping
                    .iter()
                    .find(|&map| map[0].contains(source))
                    .unwrap();
                source_map[1].nth(source_map[0].get_nth(source))
            })
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, 35);
    }
}
