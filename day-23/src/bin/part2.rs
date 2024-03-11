use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    hash::{Hash, Hasher},
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
    start_pos: Position,
    adj: RefCell<Vec<(Rc<Node>, usize)>>,
}

impl Node {
    fn from_start_position(start_pos: Position) -> Self {
        Self {
            start_pos,
            adj: RefCell::new(Vec::new()),
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("start_pos", &self.start_pos)
            .field(
                "adj",
                &self
                    .adj
                    .borrow()
                    .iter()
                    .map(|adj| (adj.0.start_pos, adj.1))
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.start_pos == other.start_pos
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

fn weighted_graph(grid: Vec<Vec<char>>, root_pos: Position) -> [Rc<Node>; 2] {
    #[allow(clippy::mutable_key_type)]
    let mut graph: HashSet<Rc<Node>> = HashSet::new();
    let root = Rc::new(Node::from_start_position(root_pos));
    let mut queue = VecDeque::from([Rc::clone(&root)]);
    let end;
    'outer: loop {
        let node = queue.pop_front().unwrap();
        let [mut i, mut j] = node.start_pos;
        let mut prev_direction = Direction::None;
        let mut weight = 1;
        loop {
            if i + 1 == grid.len() {
                end = Rc::new(Node::from_start_position([i, j]));
                node.adj.borrow_mut().push((Rc::clone(&end), weight));
                break 'outer;
            }
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
        weight += 1;
        let mut discover_adj = |next_pos| {
            let adj_raw = Node::from_start_position(next_pos);
            let adj = match graph.get(&adj_raw) {
                Some(node) => Rc::clone(node),
                None => {
                    let new_node = Rc::new(adj_raw);
                    graph.insert(Rc::clone(&new_node));
                    queue.push_back(Rc::clone(&new_node));
                    new_node
                }
            };
            adj.adj.borrow_mut().push((Rc::clone(&node), weight));
            node.adj.borrow_mut().push((adj, weight));
        };
        if grid[i][j + 1] == '>' {
            discover_adj([i, j + 2]);
        }
        if grid[i + 1][j] == 'v' {
            discover_adj([i + 2, j]);
        }
    }
    edge_contraction(graph);
    [root, end]
}

#[allow(clippy::mutable_key_type)]
fn edge_contraction(graph: HashSet<Rc<Node>>) {
    for node in graph.into_iter() {
        let adj = node.adj.borrow();
        if adj.len() != 2 {
            continue;
        }
        let new_weight = adj[0].1 + adj[1].1;

        let mut adj_adj = adj[0].0.adj.borrow_mut();
        let adj_to_node = adj_adj.iter_mut().find(|n| n.0 == node).unwrap();
        adj_to_node.0 = Rc::clone(&adj[1].0);
        adj_to_node.1 = new_weight;

        let mut adj_adj = adj[1].0.adj.borrow_mut();
        let Some(adj_to_node) = adj_adj.iter_mut().find(|n| n.0 == node) else {
            continue;
        };
        adj_to_node.0 = Rc::clone(&adj[0].0);
        adj_to_node.1 = new_weight;
    }
}

fn longest_path(root: Rc<Node>, end: Rc<Node>) -> usize {
    let mut seen = Vec::new();
    seen.push(Rc::clone(&root));
    let mut queue = vec![(Rc::clone(&root), 0, Rc::clone(&root))];
    let mut max_end_path_length = 0;
    let mut dfs_path_length = Vec::new();
    loop {
        loop {
            let (_, weight, curr) = queue.pop().unwrap();
            seen.push(Rc::clone(&curr));
            dfs_path_length.push(weight);
            let mut has_valid_adj = false;
            queue.extend(curr.adj.borrow().iter().filter_map(|(adj, adj_weight)| {
                if seen.contains(adj) {
                    None
                } else {
                    has_valid_adj = true;
                    Some((Rc::clone(&curr), *adj_weight, Rc::clone(adj)))
                }
            }));
            if !has_valid_adj {
                if curr == end {
                    max_end_path_length = max_end_path_length.max(dfs_path_length.iter().sum());
                }
                break;
            }
        }
        let Some((from, _, _)) = queue.last() else {
            break max_end_path_length;
        };
        while from != seen.last().unwrap() {
            seen.pop();
            dfs_path_length.pop();
        }
    }
}

fn part2(input: &str) -> usize {
    let [root, end] = weighted_graph(to_grid(input), [1, 1]);
    longest_path(root, end)
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
