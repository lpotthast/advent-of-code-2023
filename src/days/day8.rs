use petgraph::prelude::*;
use std::collections::BTreeMap;

pub fn part1(input: &str) -> u64 {
    let (directions, nodes) = parse(input);
    let (g, mapping) = build_graph(nodes);

    let start = *mapping.get("AAA").unwrap();
    let target = *mapping.get("ZZZ").unwrap();
    count_steps_to_reach_first_target_node(&g, start, &[target], &directions.collect::<Vec<_>>())
}

pub fn part2(input: &str) -> u64 {
    let (directions, nodes) = parse(input);
    let (g, mapping) = build_graph(nodes);

    let directions = directions.collect::<Vec<_>>();

    let target_nodes = mapping
        .keys()
        .rev()
        .filter(|k| k.ends_with('Z'))
        .map(|k| *mapping.get(*k).expect("present"))
        .collect::<Vec<_>>();

    let starting_nodes = mapping
        .keys()
        .rev()
        .filter(|k| k.ends_with('A'))
        .map(|k| *mapping.get(*k).expect("present"));

    starting_nodes
        .map(|start| count_steps_to_reach_first_target_node(&g, start, &target_nodes, &directions))
        .fold(1, |acc, next| lcm(acc, next))
}

/// Least common multiple of two positive integers. Using `gcd`.
fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

/// Greatest common divisor of two positive integers. Euclidean algorithm.
fn gcd(a: u64, b: u64) -> u64 {
    match (a, b) {
        (a, 0) => a,
        (a, b) => gcd(b, a % b),
    }
}

#[cfg(test)]
mod test {
    use super::gcd;
    use super::lcm;

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(21, 6), 42);
        assert_eq!(lcm(6, 21), 42);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(18, 48), 6);
    }
}

fn count_steps_to_reach_first_target_node(
    g: &StableGraph<(), Direction>,
    start: NodeIndex,
    targets: &[NodeIndex],
    directions: &[Direction],
) -> u64 {
    let mut current = start;
    let mut steps = 0;
    for d in directions.iter().cycle() {
        let target = g
            .edges_directed(current, Outgoing)
            .find(|e| e.weight() == d)
            .unwrap()
            .target();
        current = target;
        steps += 1;
        if targets.contains(&current) {
            break;
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
