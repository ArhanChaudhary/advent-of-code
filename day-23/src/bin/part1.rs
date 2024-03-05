use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

type Position = [usize; 2];

#[derive(PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    None,
}

type NodeRef = Rc<RefCell<Node>>;

struct Node {
    weight: Option<isize>,
    // pred: Option<NodeRef>,
    shortest_path_distance: isize,
    start_pos: Position,
    right: Option<NodeRef>,
    left: Option<NodeRef>,
    incoming_count: usize,
}

impl Node {
    fn from_start_position(start_pos: Position) -> Self {
        Self {
            weight: None,
            start_pos,
            shortest_path_distance: isize::MAX / 2,
            right: None,
            left: None,
            // pred: None,
            incoming_count: 0,
        }
    }
}

fn to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn wdag(grid: Vec<Vec<char>>, root_pos: Position) -> NodeRef {
    let mut start_position_to_node: HashMap<Position, NodeRef> = HashMap::new();
    let root = Rc::new(RefCell::new(Node::from_start_position(root_pos)));
    let mut queue = VecDeque::from([Rc::clone(&root)]);
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        let [mut i, mut j] = node.borrow().start_pos;
        let mut prev_direction = Direction::None;
        let mut weight = 1;
        while i + 1 != grid.len() {
            if prev_direction != Direction::Down && [i, j] != root_pos && grid[i - 1][j] == '.' {
                i -= 1;
                prev_direction = Direction::Up;
            } else if prev_direction != Direction::Right && grid[i][j - 1] == '.' {
                j -= 1;
                prev_direction = Direction::Left;
            } else if prev_direction != Direction::Up && grid[i + 1][j] == '.' {
                i += 1;
                prev_direction = Direction::Down;
            } else if prev_direction != Direction::Left && grid[i][j + 1] == '.' {
                j += 1;
                prev_direction = Direction::Right;
            } else {
                break;
            }
            weight += 1;
        }
        let mut node_borrow_mut = node.borrow_mut();
        if i + 1 != grid.len() {
            let mut try_from_start_position = |next_start_pos| match start_position_to_node
                .get(&next_start_pos)
            {
                Some(node) => Rc::clone(node),
                None => {
                    let new_node = Rc::new(RefCell::new(Node::from_start_position(next_start_pos)));
                    start_position_to_node.insert(next_start_pos, Rc::clone(&new_node));
                    queue.push_back(Rc::clone(&new_node));
                    new_node
                }
            };
            if grid[i][j + 1] == '>' {
                let right = try_from_start_position([i, j + 2]);
                right.borrow_mut().incoming_count += 1;
                node_borrow_mut.right = Some(right);
            }
            if grid[i + 1][j] == 'v' {
                let left = try_from_start_position([i + 2, j]);
                left.borrow_mut().incoming_count += 1;
                node_borrow_mut.left = Some(left);
            }
            weight += 1;
        }
        node_borrow_mut.weight = Some(weight);
    }
    root
}

fn kahn(root: NodeRef) -> VecDeque<NodeRef> {
    let mut topological_sorting = VecDeque::new();
    let mut no_incoming = vec![root];
    while let Some(node) = no_incoming.pop() {
        topological_sorting.push_back(Rc::clone(&node));
        let node_borrow = node.borrow();
        let mut visit = |next_node: &NodeRef| {
            if next_node.borrow().incoming_count != 0 {
                next_node.borrow_mut().incoming_count -= 1;
                if next_node.borrow().incoming_count == 0 {
                    no_incoming.push(Rc::clone(next_node));
                }
            }
        };
        if let Some(ref right) = node_borrow.right {
            visit(right);
        }
        if let Some(ref left) = node_borrow.left {
            visit(left);
        }
    }
    topological_sorting
}

fn longest_path(mut topological_sorting: VecDeque<NodeRef>) -> usize {
    topological_sorting[0].borrow_mut().shortest_path_distance = 0;
    let mut end_node: Option<NodeRef> = None;
    while let Some(node) = topological_sorting.pop_front() {
        let node_borrow = node.borrow();
        let weight = node_borrow.weight.unwrap();
        let mut has_outgoing = false;
        let mut relax_node = |next_node: &NodeRef| {
            has_outgoing = true;
            if next_node.borrow().shortest_path_distance
                > node_borrow.shortest_path_distance - weight
            {
                next_node.borrow_mut().shortest_path_distance =
                    node_borrow.shortest_path_distance - weight;
                // next_node.borrow_mut().pred = Some(Rc::clone(&node));
            }
        };
        if let Some(ref right) = node_borrow.right {
            relax_node(right);
        };
        if let Some(ref left) = node_borrow.left {
            relax_node(left);
        };
        if !has_outgoing {
            end_node = Some(Rc::clone(&node));
        }
    }
    let end_node = end_node.unwrap();
    let end_node = end_node.borrow();
    (end_node.weight.unwrap() - end_node.shortest_path_distance) as usize
}

fn part1(input: &str) -> usize {
    longest_path(kahn(wdag(to_grid(input), [1, 1])))
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
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#",
        );
        assert_eq!(result, 94);
    }
}
