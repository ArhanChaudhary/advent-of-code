use linked_hash_set::LinkedHashSet;
use std::{
    cell::{Cell, RefCell},
    collections::{HashSet, VecDeque},
    hash::Hash,
    hash::Hasher,
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

struct Node {
    weight: Cell<usize>,
    start_pos: Position,
    adj: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn from_start_position(start_pos: Position) -> Self {
        Self {
            weight: Cell::new(0),
            start_pos,
            adj: RefCell::new(Vec::new()),
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node(weight: {}, start_pos: {:?}, adj_len: {})",
            self.weight.get(),
            self.start_pos,
            self.adj.borrow().len()
        )
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.start_pos.eq(&other.start_pos)
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.start_pos.hash(state)
    }
}

fn to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn weighted_graph(grid: Vec<Vec<char>>, root_pos: Position) -> Rc<Node> {
    #[allow(clippy::mutable_key_type)]
    let mut start_position_to_node: HashSet<Rc<Node>> = HashSet::new();
    let root = Rc::new(Node::from_start_position(root_pos));
    let mut queue = VecDeque::from([Rc::clone(&root)]);
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        let [mut i, mut j] = node.start_pos;
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
        if i + 1 != grid.len() {
            let mut get_or_create = |adj| match start_position_to_node.get(&adj) {
                Some(node) => Rc::clone(node),
                None => {
                    let new_node = Rc::new(adj);
                    start_position_to_node.insert(Rc::clone(&new_node));
                    queue.push_back(Rc::clone(&new_node));
                    new_node
                }
            };
            if grid[i][j + 1] == '>' {
                let adj = get_or_create(Node::from_start_position([i, j + 2]));
                adj.adj.borrow_mut().push(Rc::clone(&node));
                node.adj.borrow_mut().push(adj);
            }
            if grid[i + 1][j] == 'v' {
                let adj = get_or_create(Node::from_start_position([i + 2, j]));
                adj.adj.borrow_mut().push(Rc::clone(&node));
                node.adj.borrow_mut().push(adj);
            }
            weight += 1;
        }
        node.weight.set(weight);
    }
    root
}

fn longest_path(root: Rc<Node>) -> usize {
    let mut seen = LinkedHashSet::new();
    seen.insert(Rc::clone(&root));
    let mut queue = vec![[Rc::clone(&root), Rc::clone(&root)]];
    let mut max_end_path_length = root.weight.get();
    let mut dfs_path_length = 0;
    loop {
        loop {
            let [_, curr] = queue.pop().unwrap();
            seen.insert(Rc::clone(&curr));
            dfs_path_length += curr.weight.get();
            let mut has_valid_adj = false;
            queue.extend(curr.adj.borrow().iter().filter_map(|adj| {
                if seen.contains(adj) {
                    None
                } else {
                    has_valid_adj = true;
                    Some([Rc::clone(&curr), Rc::clone(adj)])
                }
            }));
            if !has_valid_adj {
                let is_end_node = curr.adj.borrow().len() == 1 && curr.start_pos != root.start_pos;
                if is_end_node && dfs_path_length > max_end_path_length {
                    max_end_path_length = dfs_path_length;
                }
                break;
            }
        }
        let Some(visit_next) = queue.last() else {
            break max_end_path_length;
        };
        while visit_next[0].ne(seen.back().unwrap()) {
            dfs_path_length -= seen.pop_back().unwrap().weight.get();
        }
    }
}

fn part2(input: &str) -> usize {
    longest_path(weighted_graph(to_grid(input), [1, 1]))
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
        assert_eq!(result, 154);
    }
}
