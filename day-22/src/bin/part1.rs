use itertools::Itertools;
use std::cell::UnsafeCell;

#[derive(Debug)]
struct Point(usize, usize, usize);
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

#[derive(Debug)]
enum Orientation {
    AlongX,
    AlongY,
    AlongZ,
    None,
}
impl Orientation {
    fn from_points(p1: &Point, p2: &Point) -> Self {
        let diff_x = p1.0 != p2.0;
        let diff_y = p1.1 != p2.1;
        let diff_z = p1.2 != p2.2;
        let diff_counts = (diff_x as u8) + (diff_y as u8) + (diff_z as u8);
        if diff_counts == 0 {
            Orientation::None
        } else if diff_counts != 1 {
            panic!("Invalid orientation from points: {:?} and {:?}", p1, p2);
        } else if diff_x {
            Orientation::AlongX
        } else if diff_y {
            Orientation::AlongY
        } else if diff_z {
            Orientation::AlongZ
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug)]
struct Brick<'a> {
    upper_point: Point,
    lower_point: Point,
    orientation: Orientation,
    held_by_count: usize,
    holds: Vec<&'a Brick<'a>>,
}
impl PartialEq for Brick<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.upper_point == other.upper_point && self.lower_point == other.lower_point
    }
}
impl Brick<'_> {
    fn from(mut upper_point: Point, mut lower_point: Point) -> Self {
        if upper_point.2 < lower_point.2
            || upper_point.1 < lower_point.1
            || upper_point.0 < lower_point.0
        {
            std::mem::swap(&mut upper_point, &mut lower_point);
        }
        let orientation = Orientation::from_points(&upper_point, &lower_point);
        Brick {
            upper_point,
            lower_point,
            orientation,
            held_by_count: 0,
            holds: Vec::new(),
        }
    }

    fn contains_point(&self, point: &Point) -> bool {
        match self.orientation {
            Orientation::AlongX => {
                self.lower_point.1 == point.1
                    && self.lower_point.2 == point.2
                    && self.lower_point.0 <= point.0
                    && point.0 <= self.upper_point.0
            }
            Orientation::AlongY => {
                self.lower_point.0 == point.0
                    && self.lower_point.2 == point.2
                    && self.lower_point.1 <= point.1
                    && point.1 <= self.upper_point.1
            }
            Orientation::AlongZ => {
                self.lower_point.0 == point.0
                    && self.lower_point.1 == point.1
                    && self.lower_point.2 <= point.2
                    && point.2 <= self.upper_point.2
            }
            Orientation::None => self.lower_point == *point,
        }
    }
}

#[derive(Debug)]
struct Bricks<'a>(Vec<UnsafeCell<Brick<'a>>>);
impl<'a> Bricks<'a> {
    fn from_input(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| {
                    let mut points_iter = line.split('~');
                    let mut next_brick_point = || {
                        let mut parsed_numbers_iter = points_iter
                            .next()
                            .unwrap()
                            .split(',')
                            .map(|number_str| number_str.parse().unwrap());
                        Point(
                            parsed_numbers_iter.next().unwrap(),
                            parsed_numbers_iter.next().unwrap(),
                            parsed_numbers_iter.next().unwrap(),
                        )
                    };
                    Brick::from(next_brick_point(), next_brick_point())
                })
                .sorted_by(|a, b| a.lower_point.2.cmp(&b.lower_point.2))
                .map(UnsafeCell::new)
                .collect(),
        )
    }

    fn get_at(&self, p: &Point) -> Option<&'a mut Brick> {
        self.0
            .iter()
            .find_map(|brick_iter| {
                let brick_iter = unsafe { &mut *brick_iter.get() };
                if brick_iter.contains_point(p) {
                    Some(brick_iter)
                } else {
                    None
                }
            })
    }

    fn get_bricks_directly_below(&self, brick: &Brick) -> Option<Vec<&'a mut Brick>> {
        match brick.orientation {
            Orientation::AlongX => {
                let mut bricks_directly_below_iter = (brick.lower_point.0..=brick.upper_point.0)
                    .filter_map(|point_x| {
                        self.get_at(&Point(
                            point_x,
                            brick.lower_point.1,
                            brick.lower_point.2 - 1,
                        ))
                    })
                    .peekable();
                if bricks_directly_below_iter.peek().is_some() {
                    let mut bricks_directly_below = bricks_directly_below_iter.collect::<Vec<_>>();
                    bricks_directly_below.dedup();
                    Some(bricks_directly_below)
                } else {
                    None
                }
            }
            Orientation::AlongY => {
                let mut bricks_directly_below_iter = (brick.lower_point.1..=brick.upper_point.1)
                    .filter_map(|point_y| {
                        self.get_at(&Point(
                            brick.lower_point.0,
                            point_y,
                            brick.lower_point.2 - 1,
                        ))
                    })
                    .peekable();
                if bricks_directly_below_iter.peek().is_some() {
                    let mut bricks_directly_below_iter = bricks_directly_below_iter.collect::<Vec<_>>();
                    bricks_directly_below_iter.dedup();
                    Some(bricks_directly_below_iter)
                } else {
                    None
                }
            }
            Orientation::AlongZ | Orientation::None => self
                .get_at(&Point(
                    brick.lower_point.0,
                    brick.lower_point.1,
                    brick.lower_point.2 - 1,
                ))
                .map(|brick_right_below| vec![brick_right_below]),
        }
    }

    fn fall(&'a self) {
        for brick in &self.0 {
            let brick = unsafe { &mut *brick.get() };
            loop {
                if brick.lower_point.2 <= 1 {
                    break;
                }
                if let Some(bricks_right_below) = self.get_bricks_directly_below(brick) {
                    brick.held_by_count += bricks_right_below.len();
                    for brick_right_below in bricks_right_below {
                        brick_right_below.holds.push(brick);
                    }
                    break;
                }
                brick.upper_point.2 -= 1;
                brick.lower_point.2 -= 1;
            }
        }
    }

    fn solve(&self) -> usize {
        self.0
            .iter()
            .filter(|brick| {
                let brick = unsafe { &*brick.get() };
                brick.holds.iter().all(|held_by_this| {
                    if held_by_this.held_by_count == 0 {
                        panic!("invalid held by count, {:#?}", brick);
                    }
                    held_by_this.held_by_count > 1
                })
            })
            .count()
    }
}

fn part1(input: &str) -> usize {
    let bricks = Bricks::from_input(input);
    bricks.fall();
    bricks.solve()
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn part1_test() {
        let result = part1("\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9",
        );
        assert_eq!(result, 5);
    }
}
