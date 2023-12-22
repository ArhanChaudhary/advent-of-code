fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, Clone, Copy)]
struct Interval {
    start: usize,
    end: usize,
}

impl Interval {
    fn new(start: usize, end: usize) -> Interval {
        Interval { start, end }
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

fn part2(input: &str) -> usize {
    let mut input_lines = input.lines();
    let binding = input_lines
        .next()
        .unwrap()
        .chars()
        .skip_while(|c| *c != ':')
        .skip(1)
        .collect::<String>();
    let binding = binding.split_whitespace().collect::<Vec<&str>>();
    let seeds = binding.chunks(2).flat_map(|pair| {
        pair[0].parse::<usize>().unwrap()
            ..pair[1].parse::<usize>().unwrap() + pair[0].parse::<usize>().unwrap()
    });
    // .map(|number| number.parse().unwrap())
    let _ = input_lines.by_ref().next();
    let mut mappings: Vec<Vec<[Interval; 2]>> = Vec::new();
    loop {
        let _ = input_lines.by_ref().next();
        let mut mapping: Vec<[Interval; 2]> = input_lines
            .by_ref()
            .take_while(|&line| !line.is_empty())
            .map(|line| {
                let line_numbers: Vec<usize> = line
                    .split_whitespace()
                    .map(|number| number.parse::<usize>().unwrap())
                    .collect();
                [
                    Interval::new(line_numbers[1], line_numbers[1] + line_numbers[2]),
                    Interval::new(line_numbers[0], line_numbers[0] + line_numbers[2]),
                ]
            })
            .collect();
        if mapping.is_empty() {
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
                        Interval::new(prev_end, curr_start),
                        Interval::new(prev_end, curr_start),
                    ],
                );
                i += 1;
            }
            prev_end = dest.end();
            i += 1;
        }
        mapping.push([
            Interval::new(prev_end, usize::max_value()),
            Interval::new(prev_end, usize::max_value()),
        ]);
        mapping.sort_by(|r, r2| r[0].start().cmp(&r2[0].start()));
        mappings.push(mapping);
    }
    let mut count: u64 = 0;
    seeds
        .map(|seed| {
            count += 1;
            if count % 10000000 == 0 {
                dbg!(count);
            }
            mappings.iter().fold(seed, |source, mapping| {
                let source_map = mapping.iter().find(|&map| map[0].contains(source)).unwrap();
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
    fn part2_test() {
        let result = part2(
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
        assert_eq!(result, 46);
    }
}
