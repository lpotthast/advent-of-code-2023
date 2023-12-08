use petgraph::prelude::*;
use std::collections::BTreeMap;

pub fn part1(input: &str) -> u64 {
    let (directions, nodes) = parse(input);
    let (g, mapping) = build_graph(nodes);

    let zzz = *mapping.get("ZZZ").unwrap();
    let mut current = *mapping.get("AAA").unwrap();

    let mut steps = 0;
    for d in directions.collect::<Vec<_>>().iter().cycle() {
        let target = g
            .edges_directed(current, Outgoing)
            .find(|e| e.weight() == d)
            .unwrap()
            .target();
        current = target;
        steps += 1;
        if current == zzz {
            break;
        }
    }
    steps
}

pub fn part2(input: &str) -> u64 {
    let (directions, nodes) = parse(input);
    let (g, mapping) = build_graph(nodes);

    let zzzs = mapping
        .keys()
        .rev()
        .filter(|k| k.ends_with('Z'))
        .map(|k| *mapping.get(*k).expect("present"))
        .collect::<Vec<_>>();

    tracing::info!(?zzzs);

    let mut current_nodes = mapping
        .keys()
        .rev()
        .filter(|k| k.ends_with('A'))
        .map(|k| *mapping.get(*k).expect("present"))
        .collect::<Vec<_>>();

    tracing::info!(?current_nodes);

    let mut steps = 0;
    for d in directions.collect::<Vec<_>>().iter().cycle() {
        for current in &mut current_nodes {
            let target = g
                .edges_directed(*current, Outgoing)
                .find(|e| e.weight() == d)
                .unwrap()
                .target();
            *current = target;
        }
        steps += 1;
        let ending_on_z = current_nodes.iter().filter(|c| zzzs.contains(c)).count();
        // tracing::info!(?current_nodes, ending_on_z);
        if ending_on_z == current_nodes.len() {
            break;
        }
        if ending_on_z > 3 {
            tracing::info!(?current_nodes, ending_on_z, "matching");
        }
    }
    steps
}

fn build_graph<'a>(
    nodes: impl Iterator<Item = NodeWithEdges<'a>> + 'a,
) -> (StableGraph<(), Direction>, BTreeMap<&'a str, NodeIndex>) {
    fn get_or_insert<'a>(
        node: &'a str,
        mapping: &mut BTreeMap<&'a str, NodeIndex>,
        g: &mut StableGraph<(), Direction>,
    ) -> NodeIndex {
        match mapping.get(node) {
            Some(i) => *i,
            None => {
                let i = g.add_node(());
                mapping.insert(node, i);
                i
            }
        }
    }
    let mut g: StableGraph<(), Direction> = StableGraph::new();
    let mut mapping: BTreeMap<&str, NodeIndex> = BTreeMap::new();
    for NodeWithEdges { source, left, right } in nodes {
        let i_source = get_or_insert(source, &mut mapping, &mut g);
        let i_left = get_or_insert(left, &mut mapping, &mut g);
        let i_right = get_or_insert(right, &mut mapping, &mut g);
        g.add_edge(i_source, i_left, Direction::L);
        g.add_edge(i_source, i_right, Direction::R);
    }
    (g, mapping)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    L,
    R,
}

#[derive(Debug, Clone, Copy)]
struct NodeWithEdges<'a> {
    source: &'a str,
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
        let (source, rest) = line
            .split_once('=')
            .map(|(a, b)| (a.trim_end(), b.trim_start()))
            .expect("=");
        let (left, right) = rest
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(',')
            .map(|(a, b)| (a.trim(), b.trim()))
            .expect(",");
        NodeWithEdges { source, left, right }
    });

    (dirs, nodes)
}
