use std::{
    cell::{OnceCell, RefCell},
    collections::{HashMap, HashSet, VecDeque},
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    name: String,
    parents: RefCell<Vec<Weak<Node>>>,
    children: OnceCell<Vec<Rc<Node>>>,
}

fn create_graph(input: &str) -> Vec<Rc<Node>> {
    // By using graph-tool to visualize the graph, I was able
    // to establish that the three edges to be cut are:
    // bvc: rsm
    // bkm: ldk
    // zmq: pgh
    let mut graph: HashMap<&str, Rc<Node>> = HashMap::new();
    for line in input.lines() {
        let mut split = line.split(' ');
        let name = &split.next().unwrap()[..3];
        let curr = match graph.get(name) {
            Some(existing) => Rc::clone(existing),
            None => {
                let new = Rc::new(Node {
                    name: name.to_owned(),
                    parents: RefCell::default(),
                    children: OnceCell::new(),
                });
                graph.insert(name, Rc::clone(&new));
                new
            }
        };
        let mut adj = Vec::new();
        for adj_name in split {
            let adj_node = match graph.get(adj_name) {
                Some(existing) => Rc::clone(existing),
                None => {
                    let adj_node = Rc::new(Node {
                        name: adj_name.to_owned(),
                        parents: RefCell::default(),
                        children: OnceCell::new(),
                    });
                    graph.insert(adj_name, Rc::clone(&adj_node));
                    adj_node
                }
            };
            adj_node.parents.borrow_mut().push(Rc::downgrade(&curr));
            adj.push(adj_node);
        }
        curr.children.set(adj).unwrap();
    }
    graph.into_values().collect()
}

fn cluster_sizes(graph: Vec<Rc<Node>>) -> (usize, usize) {
    dbg!(graph.len());
    let node_count = graph.len();
    let mut queue = VecDeque::from([Rc::clone(&graph[0])]);
    let mut seen: HashSet<String> = HashSet::new();
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        if seen.insert(curr.name.clone()) {
            queue.extend(
                curr.parents
                    .borrow()
                    .iter()
                    .map(|parent| parent.upgrade().unwrap()),
            );
            if let Some(children) = curr.children.get() {
                queue.extend(children.iter().cloned());
            }
        }
    }
    let size1 = seen.len();
    dbg!((size1, node_count - size1))
}

fn part1(input: &str) -> usize {
    let (size1, size2) = cluster_sizes(create_graph(input));
    size1 * size2
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
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr",
        );
        assert_eq!(result, 54);
    }
}
