use petgraph::{stable_graph::StableGraph, visit::EdgeRef};
use std::collections::BTreeMap;

pub fn part1(input: &str) -> u64 {
    let (directions, nodes) = parse(input);

    let directions: Vec<Direction> = directions.collect();
    let n = nodes.collect::<Vec<_>>();

    //tracing::info!(?directions, ?n);

    // let mut g: StableGraph<&str, &str> = StableGraph::from_edges(n.into_iter().map(|n| (n.left, n.right)));
    let mut g: StableGraph<(), Direction> = StableGraph::new();

    let mut mapping = BTreeMap::new();

    for node in n {
        let i_source = match mapping.get(node.node) {
            Some(i) => *i,
            None => {
                let i = g.add_node(());
                mapping.insert(node.node, i);
                i
            }
        };
        let i_left = match mapping.get(node.left) {
            Some(i) => *i,
            None => {
                let i = g.add_node(());
                mapping.insert(node.left, i);
                i
            }
        };
        let i_right = match mapping.get(node.right) {
            Some(i) => *i,
            None => {
                let i = g.add_node(());
                mapping.insert(node.right, i);
                i
            }
        };

        g.add_edge(i_source, i_left, Direction::L);
        g.add_edge(i_source, i_right, Direction::R);
    }

    //tracing::info!(?g);

    let zzz = *mapping.get("ZZZ").unwrap();
    let mut current = *mapping.get("AAA").unwrap();

    let mut steps = 0;
    for d in directions.iter().cycle() {
        let target = g
            .edges_directed(current, petgraph::Direction::Outgoing)
            .find(|e| e.weight() == d)
            .unwrap()
            .target();
        current = target;
        steps += 1;
        if current == zzz {
            break;
        }
    }

    //tracing::info!(?current, steps, "Found end.");

    steps
}

pub fn part2(input: &str) -> u64 {
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    L,
    R,
}

#[derive(Debug, Clone, Copy)]
struct NodeWithEdges<'a> {
    node: &'a str,
    left: &'a str,
    right: &'a str,
}

fn parse(
    input: &str,
) -> (
    impl Iterator<Item = Direction> + '_,
    impl Iterator<Item = NodeWithEdges<'_>> + '_,
) {
    let mut lines = input.lines();
    let dirs = lines.next().expect("direction information").chars().map(|c| match c {
        'L' => Direction::L,
        'R' => Direction::R,
        _ => panic!("Unexpected direction character"),
    });

    let nodes = lines.filter(|l| !l.is_empty()).map(|line| {
        let (node, rest) = line
            .split_once('=')
            .map(|(a, b)| (a.trim_end(), b.trim_start()))
            .expect("=");
        let (left, right) = rest
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(',')
            .map(|(a, b)| (a.trim(), b.trim()))
            .expect(",");
        NodeWithEdges { node, left, right }
    });

    (dirs, nodes)
}
