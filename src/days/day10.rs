use petgraph::prelude::*;
use std::collections::BTreeMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type Graph = StableGraph<(), (), Undirected>;

pub fn part1(input: &str) -> u64 {
    let mut tile_map = parse_input(input);
    let (mut g, mut mapping, (start_row, start_col)) = build_graph(&tile_map);

    tracing::info!(start_row, start_col);
    //tracing::info!(?g);
    //tracing::info!(?tile_map);

    // For each possible pipe which would connect both ends:
    // Insert pipe and
    // Check resulting path length is cyclic and store length.

    let candidates = {
        let start = tile_map.row(start_row).col(start_col);
        tracing::info!(?start);
        Pipe::iter()
            .filter(|pipe| match pipe {
                Pipe::Vertical => {
                    start.above().is_pipe_and(Pipe::facing_south) && start.below().is_pipe_and(Pipe::facing_north)
                }
                Pipe::Horizontal => {
                    start.left().is_pipe_and(Pipe::facing_east) && start.right().is_pipe_and(Pipe::facing_west)
                }
                Pipe::NorthEast => {
                    start.above().is_pipe_and(Pipe::facing_south) && start.right().is_pipe_and(Pipe::facing_west)
                }
                Pipe::NorthWest => {
                    start.above().is_pipe_and(Pipe::facing_south) && start.left().is_pipe_and(Pipe::facing_east)
                }
                Pipe::SouthWest => {
                    start.below().is_pipe_and(Pipe::facing_north) && start.left().is_pipe_and(Pipe::facing_east)
                }
                Pipe::SouthEast => {
                    start.below().is_pipe_and(Pipe::facing_north) && start.right().is_pipe_and(Pipe::facing_west)
                }
            })
            .collect::<Vec<_>>()
    };

    tracing::info!(?candidates);

    for candidate in candidates {
        tile_map.update(start_row, start_col, Tile::Pipe(candidate));
        let start = tile_map.row(start_row).col(start_col);
        insert_node(&start, &mut mapping, &mut g, true);
    }

    //tracing::info!(?g);
    ////tracing::info!(?tile_map);

    let start_node = mapping[&[start_row, start_col]];

    let path = walk(&g, start_node);
    tracing::info!(?path);

    //let a = petgraph::algo::astar(&g, start_node, |n| n == start_node, |_e| 0, |_e| 0);
    //tracing::info!(?a);
    //
    //for p in petgraph::algo::simple_paths::all_simple_paths::<Vec<_>, _>(&g, start_node, start_node, 1, None) {
    //    tracing::info!(?p);
    //}
    (path.len() / 2) as u64
}

pub fn part2(input: &str) -> u64 {
    let mut tile_map = parse_input(input);
    let (mut g, mut mapping, (start_row, start_col)) = build_graph(&tile_map);

    let mut rev_mapping = BTreeMap::new();
    for (k, v) in &mapping {
        rev_mapping.insert(*v, *k);
    }

    tracing::info!(start_row, start_col);
    //tracing::info!(?g);
    //tracing::info!(?tile_map);

    // For each possible pipe which would connect both ends:
    // Insert pipe and
    // Check resulting path length is cyclic and store length.

    let candidates = {
        let start = tile_map.row(start_row).col(start_col);
        tracing::info!(?start);
        Pipe::iter()
            .filter(|pipe| match pipe {
                Pipe::Vertical => {
                    start.above().is_pipe_and(Pipe::facing_south) && start.below().is_pipe_and(Pipe::facing_north)
                }
                Pipe::Horizontal => {
                    start.left().is_pipe_and(Pipe::facing_east) && start.right().is_pipe_and(Pipe::facing_west)
                }
                Pipe::NorthEast => {
                    start.above().is_pipe_and(Pipe::facing_south) && start.right().is_pipe_and(Pipe::facing_west)
                }
                Pipe::NorthWest => {
                    start.above().is_pipe_and(Pipe::facing_south) && start.left().is_pipe_and(Pipe::facing_east)
                }
                Pipe::SouthWest => {
                    start.below().is_pipe_and(Pipe::facing_north) && start.left().is_pipe_and(Pipe::facing_east)
                }
                Pipe::SouthEast => {
                    start.below().is_pipe_and(Pipe::facing_north) && start.right().is_pipe_and(Pipe::facing_west)
                }
            })
            .collect::<Vec<_>>()
    };

    tracing::info!(?candidates);

    for candidate in candidates {
        tile_map.update(start_row, start_col, Tile::Pipe(candidate));
        let start = tile_map.row(start_row).col(start_col);
        insert_node(&start, &mut mapping, &mut g, true);
    }

    //tracing::info!(?g);
    ////tracing::info!(?tile_map);

    let start_node = mapping[&[start_row, start_col]];

    let path = walk(&g, start_node);
    tracing::info!(?path);

    let path_tiles = path.iter().map(|idx| rev_mapping.get(idx).unwrap()).collect::<Vec<_>>();

    for r in 1..tile_map.inner.len() - 1 {
        for c in 1..tile_map.inner[0].len() - 1 {
            if !path_tiles.contains(&&[r, c]) {
                tile_map.update(r, c, Tile::Ground);
            }
        }
    }

    tracing::info!(?tile_map);

    fn is_inner_tile() -> bool {
        false
    }

    for r in 1..tile_map.inner.len() - 1 {
        for c in 1..tile_map.inner[0].len() - 1 {
            let t = tile_map.row(r).col(c);
            match t.itself() {
                _ => is_inner_tile(),
            }
        }
    }

    //let a = petgraph::algo::astar(&g, start_node, |n| n == start_node, |_e| 0, |_e| 0);
    //tracing::info!(?a);
    //
    //for p in petgraph::algo::simple_paths::all_simple_paths::<Vec<_>, _>(&g, start_node, start_node, 1, None) {
    //    tracing::info!(?p);
    //}
    (path.len() / 2) as u64
}

fn walk(g: &Graph, start: NodeIndex) -> Vec<NodeIndex> {
    let mut last_edge_taken = None;
    let mut n = start;

    let mut path = Vec::new();

    loop {
        //tracing::warn!(?path);
        let mut es = g.edges(n);
        let first = es.next().unwrap();
        let second = es.next().unwrap();
        assert_eq!(None, es.next());

        let e = match last_edge_taken {
            Some(last) => match first == last {
                true => second,
                false => first,
            },
            None => first,
        };
        last_edge_taken = Some(e);

        n = e.target();

        path.push(n);

        if n == start {
            break;
        }
    }

    path
}

fn get_or_insert(node: [usize; 2], mapping: &mut BTreeMap<[usize; 2], NodeIndex>, g: &mut Graph) -> NodeIndex {
    match mapping.get(&node) {
        Some(i) => *i,
        None => {
            let i = g.add_node(());
            mapping.insert(node, i);
            i
        }
    }
}

fn build_graph(tile_map: &TileMap) -> (Graph, BTreeMap<[usize; 2], NodeIndex>, (usize, usize)) {
    let mut g: Graph = StableGraph::with_capacity(0, 0);
    let mut mapping: BTreeMap<[usize; 2], NodeIndex> = BTreeMap::new();
    let mut start = None;

    for row in tile_map.rows() {
        for cell in row.cells() {
            insert_node(&cell, &mut mapping, &mut g, false);

            match cell.itself() {
                Tile::Pipe(_) => {}
                Tile::Start => start = Some((cell.row, cell.col)), // TODO assert not already set!
                Tile::Ground => {}
            }
        }
    }
    (g, mapping, start.expect("start"))
}

fn insert_node<'a>(
    cell: &TileMapCell<'a>,
    mut mapping: &mut BTreeMap<[usize; 2], NodeIndex>,
    mut g: &mut Graph,
    all_sides: bool,
) {
    // Add self node.
    let i_source = get_or_insert([cell.row, cell.col], mapping, g);

    // Add edges.
    let m = cell.itself();
    let connects_with_top = cell.above().is_pipe_and(Pipe::facing_south) && m.is_pipe_and(Pipe::facing_north);
    let connects_with_left = cell.left().is_pipe_and(Pipe::facing_east) && m.is_pipe_and(Pipe::facing_west);
    let connects_with_right = cell.right().is_pipe_and(Pipe::facing_west) && m.is_pipe_and(Pipe::facing_east);
    let connects_with_bottom = cell.below().is_pipe_and(Pipe::facing_north) && m.is_pipe_and(Pipe::facing_south);

    if all_sides {
        if connects_with_top {
            let i_top = get_or_insert([cell.row - 1, cell.col], &mut mapping, &mut g);
            g.add_edge(i_source, i_top, ());
        }

        if connects_with_left {
            let i_left = get_or_insert([cell.row, cell.col - 1], &mut mapping, &mut g);
            g.add_edge(i_source, i_left, ());
        }
    }
    if connects_with_right {
        let i_right = get_or_insert([cell.row, cell.col + 1], &mut mapping, &mut g);
        g.add_edge(i_source, i_right, ());
    }
    if connects_with_bottom {
        let i_bottom = get_or_insert([cell.row + 1, cell.col], &mut mapping, &mut g);
        g.add_edge(i_source, i_bottom, ());
    }
}

#[derive(Debug, Clone, Copy, EnumIter)]
enum Pipe {
    /// '|', a vertical pipe connecting north and south.
    Vertical,
    /// '-', a horizontal pipe connecting east and west.
    Horizontal,
    /// 'L', a 90-degree bend connecting north and east.
    NorthEast,
    /// 'J', a 90-degree bend connecting north and west.
    NorthWest,
    /// '7', a 90-degree bend connecting south and west.
    SouthWest,
    /// 'F', a 90-degree bend connecting south and east.
    SouthEast,
}

impl Pipe {
    fn facing_north(self) -> bool {
        match self {
            Pipe::Vertical | Pipe::NorthEast | Pipe::NorthWest => true,
            Pipe::Horizontal | Pipe::SouthWest | Pipe::SouthEast => false,
        }
    }
    fn facing_east(self) -> bool {
        match self {
            Pipe::Horizontal | Pipe::NorthEast | Pipe::SouthEast => true,
            Pipe::Vertical | Pipe::SouthWest | Pipe::NorthWest => false,
        }
    }
    fn facing_south(self) -> bool {
        match self {
            Pipe::Vertical | Pipe::SouthWest | Pipe::SouthEast => true,
            Pipe::Horizontal | Pipe::NorthEast | Pipe::NorthWest => false,
        }
    }
    fn facing_west(self) -> bool {
        match self {
            Pipe::Horizontal | Pipe::SouthWest | Pipe::NorthWest => true,
            Pipe::Vertical | Pipe::NorthEast | Pipe::SouthEast => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Pipe(Pipe),
    Start,
    Ground,
}

impl Tile {
    fn is_pipe_and(self, pred: impl Fn(Pipe) -> bool) -> bool {
        self.pipe().map(pred).unwrap_or(false)
    }

    fn pipe(self) -> Option<Pipe> {
        match self {
            Tile::Pipe(pipe) => Some(pipe),
            Tile::Start | Tile::Ground => None,
        }
    }
}

#[derive(Debug)]
struct TileMap {
    inner: Vec<Vec<Tile>>,
}

impl TileMap {
    fn rows(&self) -> impl Iterator<Item = TileMapRow<'_>> {
        self.inner
            .windows(3) // This will skip our padding / border on top and bottom!
            .flat_map(<&[Vec<Tile>; 3]>::try_from)
            .enumerate()
            .map(|(row, it)| TileMapRow {
                above: it[0].as_slice(),
                current: it[1].as_slice(),
                below: it[2].as_slice(),
                row: row + 1,
            })
    }
    fn row(&self, row_idx: usize) -> TileMapRow<'_> {
        self.rows().skip(row_idx - 1).next().expect("valid index")
    }
    fn update(&mut self, row: usize, col: usize, tile: Tile) {
        tracing::warn!(cell = ?self.inner[row][col], "before");
        self.inner[row][col] = tile;
        tracing::warn!(cell = ?self.inner[row][col], "after");
    }

    //fn walk(&self, start_row: usize, start_col: usize) {
    //    let mut seen = Vec::new();
    //
    //    let r = start_row;
    //    let c = start_col;
    //
    //    while r !=  start_row && c != start_col && seen.is_empty() {
    //        let cell = self.row(r).col(c);
    //        match cell.itself().pipe() {
    //            Some(p) => match p {
    //                Pipe::Vertical => todo!(),
    //                Pipe::Horizontal => todo!(),
    //                Pipe::NorthEast => todo!(),
    //                Pipe::NorthWest => todo!(),
    //                Pipe::SouthWest => todo!(),
    //                Pipe::SouthEast => todo!(),
    //            },
    //            None => panic!("Reached ground node..."),
    //        }
    //    }
    //
    //
    //}
}

#[derive(Debug, Clone, Copy)]
struct TileMapRow<'a> {
    above: &'a [Tile],
    current: &'a [Tile],
    below: &'a [Tile],
    row: usize,
}

impl<'a> TileMapRow<'a> {
    fn cells(self) -> impl Iterator<Item = TileMapCell<'a>> + 'a {
        (1..self.current.len() - 1).map(move |col| self.col(col))
    }

    fn col(self, col: usize) -> TileMapCell<'a> {
        TileMapCell {
            above: self.above,
            current: self.current,
            below: self.below,
            row: self.row,
            col,
        }
    }
}

#[derive(Debug)]
struct TileMapCell<'a> {
    above: &'a [Tile],
    current: &'a [Tile],
    below: &'a [Tile],
    row: usize,
    col: usize,
}

impl<'a> TileMapCell<'a> {
    fn above(&self) -> Tile {
        self.above[self.col]
    }
    fn left(&self) -> Tile {
        self.current[self.col - 1]
    }
    fn itself(&self) -> Tile {
        self.current[self.col]
    }
    fn right(&self) -> Tile {
        self.current[self.col + 1]
    }
    fn below(&self) -> Tile {
        self.below[self.col]
    }
}

fn parse_input(input: &str) -> TileMap {
    let mut input = [Vec::new()]
        .into_iter()
        .chain(input.lines().map(|line| {
            [Tile::Ground]
                .into_iter()
                .chain(parse_tiles(line).chain([Tile::Ground]))
                .collect::<Vec<_>>()
        }))
        .chain([Vec::new()])
        .collect::<Vec<_>>();

    let input_len = input.len();
    let new_len = input[1].len();
    input[0].resize(new_len, Tile::Ground);
    input[input_len - 1].resize(new_len, Tile::Ground);

    TileMap { inner: input }
}

fn parse_tiles(line: &str) -> impl Iterator<Item = Tile> + '_ {
    line.chars().map(|c| match c {
        '|' => Tile::Pipe(Pipe::Vertical),
        '-' => Tile::Pipe(Pipe::Horizontal),
        'L' => Tile::Pipe(Pipe::NorthEast),
        'J' => Tile::Pipe(Pipe::NorthWest),
        '7' => Tile::Pipe(Pipe::SouthWest),
        'F' => Tile::Pipe(Pipe::SouthEast),
        '.' => Tile::Ground,
        'S' => Tile::Start,
        other => panic!("Unexpected tile: '{other}'"),
    })
}
