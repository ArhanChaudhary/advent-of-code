#![allow(clippy::mutable_key_type)]

use itertools::Itertools;
use std::{
    cell::{Cell, RefCell},
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    hash::{Hash, Hasher},
};

impl std::fmt::Debug for BrickEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.x, self.y, self.z.get())
    }
}

impl std::fmt::Debug for Brick<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.lower_end, self.upper_end)
    }
}

#[derive(PartialEq)]
struct BrickEnd {
    x: usize,
    y: usize,
    z: Cell<usize>,
}

#[derive(Debug, PartialEq)]
enum Orientation {
    AlongX,
    AlongY,
    AlongZ,
    None,
}
impl Orientation {
    fn from_brick_ends(e1: &BrickEnd, e2: &BrickEnd) -> Self {
        let diff_x = e1.x != e2.x;
        let diff_y = e1.y != e2.y;
        let diff_z = e1.z != e2.z;
        let diff_counts = (diff_x as u8) + (diff_y as u8) + (diff_z as u8);
        if diff_counts == 0 {
            Orientation::None
        } else if diff_counts != 1 {
            panic!("Invalid orientation from points: {:?} and {:?}", e1, e2);
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

struct Brick<'a> {
    id: usize,
    upper_end: BrickEnd,
    lower_end: BrickEnd,
    orientation: Orientation,
    held_by: RefCell<Vec<&'a Brick<'a>>>,
    holds: RefCell<Vec<&'a Brick<'a>>>,
}
impl PartialEq for Brick<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.upper_end == other.upper_end && self.lower_end == other.lower_end
    }
}
impl Eq for Brick<'_> {}
impl Hash for Brick<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}
impl PartialOrd for Brick<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Brick<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.upper_end.z.get().cmp(&self.upper_end.z.get())
    }
}
impl Brick<'_> {
    fn from(mut upper_end: BrickEnd, mut lower_end: BrickEnd, id: usize) -> Self {
        if upper_end.z < lower_end.z || upper_end.y < lower_end.y || upper_end.x < lower_end.x {
            std::mem::swap(&mut upper_end, &mut lower_end);
        }
        let orientation = Orientation::from_brick_ends(&upper_end, &lower_end);
        Brick {
            id,
            upper_end,
            lower_end,
            orientation,
            held_by: Default::default(),
            holds: Default::default(),
        }
    }
    fn point_xys(&self) -> Box<dyn Iterator<Item = [usize; 2]> + '_> {
        match self.orientation {
            Orientation::AlongX => Box::new(
                (self.lower_end.x..=self.upper_end.x).map(|point_x| [point_x, self.lower_end.y]),
            ),
            Orientation::AlongY => Box::new(
                (self.lower_end.y..=self.upper_end.y).map(|point_y| [self.lower_end.x, point_y]),
            ),
            Orientation::AlongZ | Orientation::None => {
                Box::new(std::iter::once([self.lower_end.x, self.lower_end.y]))
            }
        }
    }
}

struct Bricks<'a>(Vec<Brick<'a>>);
impl Bricks<'_> {
    fn from_input(input: &str) -> Self {
        Self(
            input
                .lines()
                .enumerate()
                .map(|(i, line)| {
                    let mut points_iter = line.split('~');
                    let mut next_brick_point = || {
                        let mut parsed_numbers_iter = points_iter
                            .next()
                            .unwrap()
                            .split(',')
                            .map(|number_str| number_str.parse().unwrap());
                        BrickEnd {
                            x: parsed_numbers_iter.next().unwrap(),
                            y: parsed_numbers_iter.next().unwrap(),
                            z: Cell::new(parsed_numbers_iter.next().unwrap()),
                        }
                    };
                    Brick::from(next_brick_point(), next_brick_point(), i)
                })
                .sorted_by(|a, b| a.lower_end.z.cmp(&b.lower_end.z))
                .collect(),
        )
    }

    fn solve(&self) -> usize {
        self.0
            .iter()
            .map(|brick| {
                let mut queue: VecDeque<&Brick> = VecDeque::from([brick]);
                let mut fallen: HashSet<&Brick> = HashSet::new();
                let mut total_hold_count = 0;
                while !queue.is_empty() {
                    let curr = queue.pop_front().unwrap();
                    if fallen.contains(curr) {
                        continue;
                    }
                    fallen.insert(curr);
                    for brick_hold in curr.holds.borrow().iter().filter(|&&brick_hold| {
                        brick_hold
                            .held_by
                            .borrow()
                            .iter()
                            .all(|brick_hold_held_by| fallen.contains(brick_hold_held_by))
                    }) {
                        queue.push_back(brick_hold);
                        total_hold_count += 1;
                    }
                }
                total_hold_count
            })
            .sum()
    }
}

impl<'a> Bricks<'a> {
    fn fall(&'a self) {
        // it's safe to use btreeset because the order of the bricks according
        // to their z value wont change no matter how much they fall.
        let mut xy_to_bricks: HashMap<[usize; 2], BTreeSet<&Brick>> = HashMap::new();
        for brick in &self.0 {
            for point_xy in brick.point_xys() {
                xy_to_bricks.entry(point_xy).or_default().insert(brick);
            }
        }
        for brick in &self.0 {
            let mut highest_will_fall_on = 0;
            let mut will_fall_on_bricks: HashSet<&Brick> = HashSet::new();
            for point_xy in brick.point_xys() {
                let Some(&point_will_fall_on) = xy_to_bricks
                    .get(&point_xy)
                    .unwrap()
                    .range::<Brick, _>(brick..)
                    .nth(1)
                else {
                    continue;
                };
                let point_will_fall_on_height = point_will_fall_on.upper_end.z.get();
                if point_will_fall_on_height < highest_will_fall_on {
                    continue;
                }
                if point_will_fall_on_height > highest_will_fall_on {
                    will_fall_on_bricks.clear();
                    highest_will_fall_on = point_will_fall_on_height;
                }
                will_fall_on_bricks.insert(point_will_fall_on);
            }
            let diff = brick.lower_end.z.get() - highest_will_fall_on - 1;
            brick.upper_end.z.set(brick.upper_end.z.get() - diff);
            brick.lower_end.z.set(brick.lower_end.z.get() - diff);
            for will_fall_on_brick in will_fall_on_bricks {
                will_fall_on_brick.holds.borrow_mut().push(brick);
                brick.held_by.borrow_mut().push(will_fall_on_brick);
            }
        }
    }
}

fn part2(input: &str) -> usize {
    let bricks = Bricks::from_input(input);
    bricks.fall();
    bricks.solve()
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn part2_test() {
        let result = part2("\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9",
        );
        assert_eq!(result, 7);
    }
}
