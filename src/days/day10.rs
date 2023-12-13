use core::panic;
use itertools::Itertools;
use ndarray::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn part1(input: &str) -> u64 {
    let mut tile_map = parse_input(input);

    let start_replacement = find_candidates(&tile_map, tile_map.start_row, tile_map.start_col)
        .exactly_one()
        .map_err(|_| ())
        .expect("exactly one candidate");

    tile_map.inner[[tile_map.start_row, tile_map.start_col]] = Tile::Pipe(start_replacement);

    let (path, _path_map) = walk(&tile_map, tile_map.start_row, tile_map.start_col);
    (path.len() / 2) as u64
}

pub fn part2(input: &str) -> u64 {
    let mut tile_map = parse_input(input);

    let start_replacement = find_candidates(&tile_map, tile_map.start_row, tile_map.start_col)
        .exactly_one()
        .map_err(|_| ())
        .expect("exactly one candidate");

    tile_map.inner[[tile_map.start_row, tile_map.start_col]] = Tile::Pipe(start_replacement);

    let (_path, path_map) = walk(&tile_map, tile_map.start_row, tile_map.start_col);

    tile_map
        .inner
        .indexed_iter_mut()
        .filter(|(_, tile)| match tile {
            Tile::Ground => false,
            _ => true,
        })
        .for_each(|((r, c), tile)| {
            if !path_map[[r, c]] {
                *tile = Tile::Ground;
            }
        });

    tile_map
        .inner
        .indexed_iter()
        .filter(|((r, c), tile)| **tile == Tile::Ground && is_inner_tile(&tile_map, *r, *c))
        .count() as u64
}

fn find_candidates(tile_map: &TileMap, r: usize, c: usize) -> impl Iterator<Item = Pipe> + '_ {
    Pipe::iter().filter(move |pipe| match pipe {
        Pipe::Vertical => {
            tile_map
                .above(r, c)
                .map_or(false, |t| t.is_pipe_and(Pipe::facing_south))
                && tile_map
                    .below(r, c)
                    .map_or(false, |t| t.is_pipe_and(Pipe::facing_north))
        }
        Pipe::Horizontal => {
            tile_map.left(r, c).map_or(false, |t| t.is_pipe_and(Pipe::facing_east))
                && tile_map.right(r, c).map_or(false, |t| t.is_pipe_and(Pipe::facing_west))
        }
        Pipe::NorthEast => {
            tile_map
                .above(r, c)
                .map_or(false, |t| t.is_pipe_and(Pipe::facing_south))
                && tile_map.right(r, c).map_or(false, |t| t.is_pipe_and(Pipe::facing_west))
        }
        Pipe::NorthWest => {
            tile_map
                .above(r, c)
                .map_or(false, |t| t.is_pipe_and(Pipe::facing_south))
                && tile_map.left(r, c).map_or(false, |t| t.is_pipe_and(Pipe::facing_east))
        }
        Pipe::SouthWest => {
            tile_map
                .below(r, c)
                .map_or(false, |t| t.is_pipe_and(Pipe::facing_north))
                && tile_map.left(r, c).map_or(false, |t| t.is_pipe_and(Pipe::facing_east))
        }
        Pipe::SouthEast => {
            tile_map
                .below(r, c)
                .map_or(false, |t| t.is_pipe_and(Pipe::facing_north))
                && tile_map.right(r, c).map_or(false, |t| t.is_pipe_and(Pipe::facing_west))
        }
    })
}

fn is_inner_tile(tile_map: &TileMap, r: usize, c: usize) -> bool {
    [
        tile_map
            .ray_east(r, c)
            .filter(|t| {
                t.is_pipe_and(|p| match p {
                    Pipe::Vertical | Pipe::NorthEast | Pipe::NorthWest => true, // |, L, J
                    Pipe::Horizontal | Pipe::SouthWest | Pipe::SouthEast => false, // -, 7, F
                })
            })
            .count(),
        tile_map
            .ray_west(r, c)
            .filter(|t| {
                t.is_pipe_and(|p| match p {
                    Pipe::Vertical | Pipe::NorthEast | Pipe::NorthWest => true, // |, L, J
                    Pipe::Horizontal | Pipe::SouthWest | Pipe::SouthEast => false, // -, 7, F
                })
            })
            .count(),
    ]
    .into_iter()
    .all(|crossings| crossings % 2 != 0)
}

fn walk(
    tile_map: &TileMap,
    start_row: usize,
    start_col: usize,
) -> (
    Vec<(usize, usize)>,
    ArrayBase<ndarray::OwnedRepr<bool>, Dim<[usize; 2]>>,
) {
    enum Direction {
        North,
        East,
        South,
        West,
    }

    let mut from = match tile_map.inner[(start_row, start_col)] {
        Tile::Pipe(p) => match p {
            Pipe::Vertical | Pipe::SouthEast => Direction::South,
            Pipe::Horizontal | Pipe::SouthWest => Direction::West,
            Pipe::NorthEast => Direction::East,
            Pipe::NorthWest => Direction::North,
        },
        _ => panic!("expected pipe"),
    };
    let mut r = start_row;
    let mut c = start_col;

    let mut path = Vec::new();
    let mut path_map = Array2::<bool>::from_elem(tile_map.inner.raw_dim(), false);

    loop {
        path.push((r, c));
        path_map[[r, c]] = true;

        match tile_map.inner[[r, c]] {
            Tile::Pipe(p) => match p {
                Pipe::Vertical => match from {
                    Direction::North => {
                        r += 1;
                        from = Direction::North;
                    }
                    Direction::South => {
                        r -= 1;
                        from = Direction::South;
                    }
                    _ => panic!("wrong"),
                },
                Pipe::Horizontal => match from {
                    Direction::East => {
                        c -= 1;
                        from = Direction::East;
                    }
                    Direction::West => {
                        c += 1;
                        from = Direction::West;
                    }
                    _ => panic!("wrong"),
                },
                Pipe::NorthEast => match from {
                    Direction::North => {
                        c += 1;
                        from = Direction::West;
                    }
                    Direction::East => {
                        r -= 1;
                        from = Direction::South;
                    }
                    _ => panic!("wrong"),
                },
                Pipe::NorthWest => match from {
                    Direction::North => {
                        c -= 1;
                        from = Direction::East;
                    }
                    Direction::West => {
                        r -= 1;
                        from = Direction::South;
                    }
                    _ => panic!("wrong"),
                },
                Pipe::SouthWest => match from {
                    Direction::South => {
                        c -= 1;
                        from = Direction::East;
                    }
                    Direction::West => {
                        r += 1;
                        from = Direction::North;
                    }
                    _ => panic!("wrong"),
                },
                Pipe::SouthEast => match from {
                    Direction::South => {
                        c += 1;
                        from = Direction::West;
                    }
                    Direction::East => {
                        r += 1;
                        from = Direction::North;
                    }
                    _ => panic!("wrong"),
                },
            },
            _ => panic!("expected pipe"),
        }

        if r == start_row && c == start_col {
            break;
        }
    }

    (path, path_map)
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
    const fn facing_north(self) -> bool {
        match self {
            Self::Vertical | Self::NorthEast | Self::NorthWest => true,
            Self::Horizontal | Self::SouthWest | Self::SouthEast => false,
        }
    }
    const fn facing_east(self) -> bool {
        match self {
            Self::Horizontal | Self::NorthEast | Self::SouthEast => true,
            Self::Vertical | Self::SouthWest | Self::NorthWest => false,
        }
    }
    const fn facing_south(self) -> bool {
        match self {
            Self::Vertical | Self::SouthWest | Self::SouthEast => true,
            Self::Horizontal | Self::NorthEast | Self::NorthWest => false,
        }
    }
    const fn facing_west(self) -> bool {
        match self {
            Self::Horizontal | Self::SouthWest | Self::NorthWest => true,
            Self::Vertical | Self::NorthEast | Self::SouthEast => false,
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
        match self {
            Self::Pipe(pipe) => pred(pipe),
            _ => false,
        }
    }
}

#[derive(Debug)]
struct TileMap {
    inner: ArrayBase<ndarray::OwnedRepr<Tile>, Dim<[usize; 2]>>,
    start_row: usize,
    start_col: usize,
}

impl TileMap {
    fn height(&self) -> usize {
        self.inner.shape()[0]
    }

    fn width(&self) -> usize {
        self.inner.shape()[1]
    }

    fn above(&self, r: usize, c: usize) -> Option<Tile> {
        match r > 0 {
            true => Some(self.inner[[r - 1, c]]),
            false => None,
        }
    }
    fn left(&self, r: usize, c: usize) -> Option<Tile> {
        match c > 0 {
            true => Some(self.inner[[r, c - 1]]),
            false => None,
        }
    }
    fn right(&self, r: usize, c: usize) -> Option<Tile> {
        match c < self.width() {
            true => Some(self.inner[[r, c + 1]]),
            false => None,
        }
    }
    fn below(&self, r: usize, c: usize) -> Option<Tile> {
        match r < self.height() {
            true => Some(self.inner[[r + 1, c]]),
            false => None,
        }
    }

    fn ray_east(&self, row: usize, col: usize) -> impl Iterator<Item = Tile> + '_ {
        (col..self.width()).map(move |c| self.inner[[row, c]])
    }

    fn ray_west(&self, row: usize, col: usize) -> impl Iterator<Item = Tile> + '_ {
        (0..col).rev().map(move |c| self.inner[[row, c]])
    }
}

fn parse_input(input: &str) -> TileMap {
    let rows = input.lines().count();
    let cols = input.lines().next().map_or(0, |line| line.chars().count());

    let mut map: ArrayBase<ndarray::OwnedRepr<Tile>, Dim<[usize; 2]>> = Array2::from_elem((rows, cols), Tile::Ground);

    input.lines().enumerate().for_each(|(r, line)| {
        let mut num = 0;
        for (c, tile) in parse_tiles(line).enumerate() {
            map[[r, c]] = tile;
            num += 1;
        }
        assert!(num == cols, "inconsistent line length");
    });

    let (start_row, start_col) = map
        .rows()
        .into_iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter().enumerate().find_map(|(c, t)| {
                if *t == Tile::Start {
                    return Some((r, c));
                }
                None
            })
        })
        .expect("start");

    TileMap {
        inner: map,
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
