use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    to_cost: usize,
    row: usize,
    col: usize,
}

#[derive(Eq, Clone, Hash)]
struct Visit<'a> {
    node: &'a Node,
    from: Option<(Direction, usize)>,
    distance: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

type Grid = Vec<Vec<Node>>;

impl Node {
    fn neighbor<'a>(&self, direction: Direction, grid: &'a Grid) -> Option<(&'a Node, Direction)> {
        let row_count = grid.len();
        let col_count = grid[0].len();
        match direction {
            Direction::Up => {
                if self.row == 0 {
                    None
                } else {
                    Some((&grid[self.row - 1][self.col], direction))
                }
            }
            Direction::Right => {
                if self.col == col_count - 1 {
                    None
                } else {
                    Some((&grid[self.row][self.col + 1], direction))
                }
            }
            Direction::Down => {
                if self.row == row_count - 1 {
                    None
                } else {
                    Some((&grid[self.row + 1][self.col], direction))
                }
            }
            Direction::Left => {
                if self.col == 0 {
                    None
                } else {
                    Some((&grid[self.row][self.col - 1], direction))
                }
            }
        }
    }
}

impl<'a> Visit<'a> {
    fn neighbors(&self, grid: &'a Grid) -> Vec<(&'a Node, Direction)> {
        match self.from {
            Some((root_from_direction, root_from_direction_streak)) => match root_from_direction {
                Direction::Up | Direction::Down => [Direction::Right, Direction::Left],
                Direction::Right | Direction::Left => [Direction::Up, Direction::Down],
            }
            .into_iter()
            .map(|neighbor_direction| self.node.neighbor(neighbor_direction, grid))
            .chain(std::iter::once({
                if root_from_direction_streak >= 3 {
                    None
                } else {
                    self.node.neighbor(root_from_direction, grid)
                }
            }))
            .flatten()
            .collect(),
            None => [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ]
            .into_iter()
            .map(|neighbor_direction| self.node.neighbor(neighbor_direction, grid))
            .flatten()
            .collect(),
        }
    }
}

impl Ord for Visit<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Visit<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Visit<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

fn dijkstra<'a>(start: &'a Node, goal: &'a Node, grid: &'a Grid) -> usize {
    let mut visited = HashSet::new();
    let mut unvisited_nodes = BinaryHeap::new();

    unvisited_nodes.push(Visit {
        from: None,
        node: &start,
        distance: 0,
    });

    loop {
        let visiting = unvisited_nodes.pop().unwrap();
        if !visited.insert((visiting.node, visiting.from)) {
            continue;
        }

        if visiting.node == goal {
            break visiting.distance;
        }

        for (neighbor, neighbor_direction) in visiting.neighbors(grid) {
            let new_distance = visiting.distance + neighbor.to_cost;
            let direction_streak =
                visiting
                    .from
                    .map_or(1, |(from_direction, from_direction_streak)| {
                        if neighbor_direction != from_direction {
                            1
                        } else {
                            from_direction_streak + 1
                        }
                    });
            unvisited_nodes.push(Visit {
                from: Some((neighbor_direction, direction_streak)),
                node: neighbor,
                distance: new_distance,
            });
        }
    }
}

fn part1(input: &str) -> usize {
    let grid: Grid = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| Node {
                    row: i,
                    col: j,
                    to_cost: c.to_digit(10).unwrap() as usize,
                })
                .collect()
        })
        .collect();
    dijkstra(&grid[0][0], grid.last().unwrap().last().unwrap(), &grid)
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
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        );
        assert_eq!(result, 102);
    }
}
