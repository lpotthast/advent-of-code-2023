use core::panic;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn part1(input: &str) -> u64 {
    let mut tile_map = parse_input(input);

    let mut candidates = find_candidates(&tile_map, tile_map.start_row, tile_map.start_col);
    let s_replacement = candidates.next().expect("at least one valid S replacement");
    assert!(candidates.next().is_none(), "Exactly one S replacement");
    drop(candidates);

    tile_map.update(tile_map.start_row, tile_map.start_col, Tile::Pipe(s_replacement));

    let path = walk(&tile_map, tile_map.start_row, tile_map.start_col);
    (path.len() / 2) as u64
}

pub fn part2(input: &str) -> u64 {
    let mut tile_map = parse_input(input);

    let mut candidates = find_candidates(&tile_map, tile_map.start_row, tile_map.start_col);
    let s_replacement = candidates.next().expect("at least one valid S replacement");
    assert!(candidates.next().is_none(), "Exactly one S replacement");
    drop(candidates);

    tile_map.update(tile_map.start_row, tile_map.start_col, Tile::Pipe(s_replacement));

    let path = walk(&tile_map, tile_map.start_row, tile_map.start_col);

    // Set any non-path tiles to `Ground`.
    for r in 0..tile_map.inner.len() {
        for c in 0..tile_map.inner[0].len() {
            if !path.contains(&(r, c)) {
                tile_map.update(r, c, Tile::Ground);
            }
        }
    }

    let mut inner_tiles = 0;
    for r in 0..tile_map.inner.len() {
        for c in 0..tile_map.inner[0].len() {
            if let Tile::Ground = tile_map.inner[r][c] {
                if is_inner_tile(&tile_map, r, c) {
                    inner_tiles += 1;
                }
            }
        }
    }
    inner_tiles
}

fn find_candidates(tile_map: &TileMap, r: usize, c: usize) -> impl Iterator<Item = Pipe> + '_ {
    Pipe::iter()
        .filter(move |pipe| match pipe {
            Pipe::Vertical => {
                tile_map.above(r, c).map_or(false, |t| t.is_pipe_and(Pipe::facing_south)) && tile_map.below(r, c).map_or(false, |t| t.is_pipe_and(Pipe::facing_north))
            }
            Pipe::Horizontal => {
                tile_map.left(r, c).map_or(false, |t| t.is_pipe_and(Pipe::facing_east)) && tile_map.right(r, c).map_or(false, |t| t.is_pipe_and(Pipe::facing_west))
            }
            Pipe::NorthEast => {
                tile_map.above(r, c).map_or(false, |t| t.is_pipe_and(Pipe::facing_south)) && tile_map.right(r, c).map_or(false, |t| t.is_pipe_and(Pipe::facing_west))
            }
            Pipe::NorthWest => {
                tile_map.above(r, c).map_or(false, |t| t.is_pipe_and(Pipe::facing_south)) && tile_map.left(r, c).map_or(false, |t| t.is_pipe_and(Pipe::facing_east))
            }
            Pipe::SouthWest => {
                tile_map.below(r, c).map_or(false, |t| t.is_pipe_and(Pipe::facing_north)) && tile_map.left(r, c).map_or(false, |t|t.is_pipe_and(Pipe::facing_east))
            }
            Pipe::SouthEast => {
                tile_map.below(r, c).map_or(false, |t| t.is_pipe_and(Pipe::facing_north)) && tile_map.right(r, c).map_or(false, |t| t.is_pipe_and(Pipe::facing_west))
            }
        })
}

fn is_inner_tile(tile_map: &TileMap, r: usize, c: usize) -> bool {
    let cross_east = tile_map
        .ray_east(r, c)
        .filter(|t| {
            t.is_pipe_and(|p| match p {
                Pipe::Vertical => true,
                Pipe::Horizontal => false,
                Pipe::NorthEast => true,  // L
                Pipe::NorthWest => true,  // J
                Pipe::SouthWest => false, // 7
                Pipe::SouthEast => false, // F
            })
        })
        .count();

    let cross_west = tile_map
        .ray_west(r, c)
        .filter(|t| {
            t.is_pipe_and(|p| match p {
                Pipe::Vertical => true,
                Pipe::Horizontal => false,
                Pipe::NorthEast => true,  // L
                Pipe::NorthWest => true,  // J
                Pipe::SouthWest => false, // 7
                Pipe::SouthEast => false, // F
            })
        })
        .count();

    [cross_east, cross_west].iter().all(|it| *it % 2 != 0)
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn walk<'a>(tile_map: &'a TileMap, start_row: usize, start_col: usize) -> Vec<(usize, usize)> {
    let mut from = match tile_map.inner[start_row][start_col] {
        Tile::Pipe(p) => match p {
            Pipe::Vertical => Direction::South,
            Pipe::Horizontal => Direction::West,
            Pipe::NorthEast => Direction::East,
            Pipe::NorthWest => Direction::North,
            Pipe::SouthWest => Direction::West,
            Pipe::SouthEast => Direction::South,
        },
        _ => panic!("expected pipe")
    };
    let mut r = start_row;
    let mut c = start_col;

    let mut path = Vec::new();

    loop {
        path.push((r, c));

        match tile_map.inner[r][c] {
            Tile::Pipe(p) => match p {
                Pipe::Vertical => match from {
                    Direction::North => {r += 1; from = Direction::North;},
                    Direction::South => {r -= 1; from = Direction::South;},
                    _ => panic!("wrong"),
                },
                Pipe::Horizontal => match from {
                    Direction::East => {c -= 1; from = Direction::East;},
                    Direction::West => {c += 1; from = Direction::West;},
                    _ => panic!("wrong"),
                },
                Pipe::NorthEast => match from {
                    Direction::North => {c += 1; from = Direction::West;},
                    Direction::East => {r -= 1; from = Direction::South;},
                    _ => panic!("wrong"),
                },
                Pipe::NorthWest => match from {
                    Direction::North => {c -= 1; from = Direction::East;},
                    Direction::West => {r -= 1; from = Direction::South;},
                    _ => panic!("wrong"),
                },
                Pipe::SouthWest => match from {
                    Direction::South => {c -= 1; from = Direction::East;},
                    Direction::West => {r += 1; from = Direction::North;},
                    _ => panic!("wrong"),
                },
                Pipe::SouthEast => match from {
                    Direction::South => {c += 1; from = Direction::West;},
                    Direction::East => {r += 1; from = Direction::North;},
                    _ => panic!("wrong"),
                },
            },
            _ => panic!("expected pipe")
        }

        if r == start_row && c == start_col {
            break;
        }
    }

    path
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    start_row: usize,
    start_col: usize
}

impl TileMap {
    fn above(&self, r: usize, c: usize) -> Option<Tile> {
        match r > 0 {
            true => Some(self.inner[r - 1][c]),
            false => None,
        }
    }
    fn left(&self, r: usize, c: usize) -> Option<Tile> {
        match c > 0 {
            true => Some(self.inner[r][c - 1]),
            false => None,
        }
    }
    fn right(&self, r: usize, c: usize) -> Option<Tile> {
        match c < self.inner[r].len() - 1 {
            true => Some(self.inner[r][c + 1]),
            false => None,
        }
    }
    fn below(&self, r: usize, c: usize) -> Option<Tile> {
        match r < self.inner.len() - 1 {
            true => Some(self.inner[r + 1][c]),
            false => None,
        }
    }

    fn update(&mut self, row: usize, col: usize, tile: Tile) {
        self.inner[row][col] = tile;
    }

    fn ray_east(&self, row: usize, col: usize) -> impl Iterator<Item = Tile> + '_ {
        (col + 0..self.inner[row].len()).map(move |c| self.inner[row][c])
    }
    fn ray_west(&self, row: usize, col: usize) -> impl Iterator<Item = Tile> + '_ {
        (0..col).rev().map(move |c| self.inner[row][c])
    }
}

fn parse_input(input: &str) -> TileMap {
    let inner = input
        .lines()
        .map(|line| parse_tiles(line).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start_row = 0;
    let mut start_col = 0;
    for r in 0..inner.len() {
        for c in 0..inner[0].len() {
            if inner[r][c] == Tile::Start {
                start_row = r;
                start_col = c;
            }
        }
    }

    TileMap {
        inner,
        start_row,
        start_col,
    }
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
