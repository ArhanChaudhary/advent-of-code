use itertools::Itertools;
use regex::Regex;

struct Hailstone {
    pos: [f64; 3],
    vel: [f64; 3],
}

impl Hailstone {
    fn path_intersects_future(&self, other: &Self, lower_bound: f64, upper_bound: f64) -> bool {
        #[allow(unused_variables)]
        let [px1, py1, pz1] = self.pos;
        #[allow(unused_variables)]
        let [vx1, vy1, vz1] = self.vel;
        #[allow(unused_variables)]
        let [px2, py2, pz2] = other.pos;
        #[allow(unused_variables)]
        let [vx2, vy2, vz2] = other.vel;
        let m1 = vy1 / vx1;
        let m2 = vy2 / vx2;
        let intersection_x = (m1 * px1 - m2 * px2 + py2 - py1) / (m1 - m2);
        let intersection_y = m1 * intersection_x - m1 * px1 + py1;
        let intersections_in_bound = intersection_x >= lower_bound
            && upper_bound >= intersection_x
            && intersection_y >= lower_bound
            && upper_bound >= intersection_y;
        let first_is_valid = (intersection_x - px1).signum() == vx1.signum();
        let second_is_valid = (intersection_x - px2).signum() == vx2.signum();
        intersections_in_bound && first_is_valid && second_is_valid
    }
}

fn to_hailstones(input: &str) -> Vec<Hailstone> {
    let parser = Regex::new(r"(\d+), (\d+), (\d+) @ (-?\d+), (-?\d+), (-?\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let (_, numbers): (_, [&str; 6]) = parser.captures_iter(line).next().unwrap().extract();
            let mut numbers = numbers
                .into_iter()
                .map(|number| number.parse::<f64>().unwrap());
            Hailstone {
                pos: [
                    numbers.next().unwrap(),
                    numbers.next().unwrap(),
                    numbers.next().unwrap(),
                ],
                vel: [
                    numbers.next().unwrap(),
                    numbers.next().unwrap(),
                    numbers.next().unwrap(),
                ],
            }
        })
        .collect::<Vec<Hailstone>>()
}

fn solve(hailstores: Vec<Hailstone>, lower_bound: f64, upper_bound: f64) -> usize {
    hailstores
        .iter()
        .combinations(2)
        .filter(|c| {
            let first = c[0];
            let second = c[1];
            first.path_intersects_future(second, lower_bound, upper_bound)
        })
        .count()
}

fn part1(input: &str, lower_bound: f64, upper_bound: f64) -> usize {
    solve(to_hailstones(input), lower_bound, upper_bound)
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input, 200000000000000.0, 400000000000000.0);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn part1_test() {
        let result = part1("\
19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3",
            7.0,
            27.0,
        );
        assert_eq!(result, 2);
    }
}
