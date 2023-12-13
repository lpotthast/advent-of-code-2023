use ndarray::prelude::*;

pub fn part1(input: &str) -> u64 {
    Universe::from_input(input, 1, 1)
        .galaxy_combinations()
        .map(Galaxy::manhattan_distance_tuple)
        .sum()
}

pub fn part2_expand_9(input: &str) -> u64 {
    Universe::from_input(input, 9, 9)
        .galaxy_combinations()
        .map(Galaxy::manhattan_distance_tuple)
        .sum()
}

pub fn part2_expand_99(input: &str) -> u64 {
    Universe::from_input(input, 99, 99)
        .galaxy_combinations()
        .map(Galaxy::manhattan_distance_tuple)
        .sum()
}

pub fn part2_expand_999_999(input: &str) -> u64 {
    Universe::from_input(input, 999_999, 999_999)
        .galaxy_combinations()
        .map(Galaxy::manhattan_distance_tuple)
        .sum()
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Galaxy>,
}

impl Universe {
    /// Parse the universe supplied through `input`. Must only contain galaxies (`#`s) and empty space (`.`s).
    ///
    /// * `input` - The full input to parse.
    /// * `empty_row_expansion` - Each empty row (without any galaxies) is interpreted as having a width of `1 + empty_row_expansion` rows.
    /// * `empty_col_expansion` - Each empty col (without any galaxies) is interpreted as having a height of `1 + empty_col_expansion` columns.
    fn from_input(input: &str, empty_row_expansion: usize, empty_col_expansion: usize) -> Self {
        let rows = input.lines().count();
        let cols = input.lines().next().map_or(0, |line| line.chars().count());

        // Find empty rows and columns.
        let mut empty_rows = Array::from_elem(rows, true);
        let mut empty_cols = Array::from_elem(cols, true);
        for (r, line) in input.lines().enumerate() {
            for (c, char) in line.chars().enumerate() {
                match char {
                    '.' => {}
                    _ => {
                        empty_rows[r] = false;
                        empty_cols[c] = false;
                    }
                }
            }
        }

        // Read galaxies.
        let mut galaxies = Vec::new();
        for (r, line) in input.lines().enumerate() {
            for (c, char) in line.chars().enumerate() {
                match char {
                    '#' => {
                        let num_empty_rows_before = empty_rows.iter().take(r).filter(|e| **e).count();
                        let num_empty_cols_before = empty_cols.iter().take(c).filter(|e| **e).count();

                        let row_expansion = num_empty_rows_before * empty_row_expansion;
                        let col_expansion = num_empty_cols_before * empty_col_expansion;

                        galaxies.push(Galaxy {
                            x: c + col_expansion,
                            y: r + row_expansion,
                        });
                    }
                    _ => {}
                }
            }
        }

        Self { galaxies }
    }

    fn galaxy_combinations(&self) -> impl Iterator<Item = (Galaxy, Galaxy)> + '_ {
        GalaxyCombinations {
            galaxies: &self.galaxies,
            i: 0,
            j: 0,
        }
        /*
        // We could have used the itertools crate, but this is ~30 times slower than our manual "iterate over galaxy tuples" approach,
        // The itertools `combinations` function provides features we do not need: works on iterators, allows arbitrary sizes, ...
        self.galaxies.into_iter().combinations(2).map(|combinations| {
            let first = *combinations.get(0).unwrap();
            let second = *combinations.get(1).unwrap();
            (first, second)
        })
        */
    }
}

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    fn manhattan_distance_tuple((g1, g2): (Galaxy, Galaxy)) -> u64 {
        Self::manhattan_distance(g1, g2)
    }

    fn manhattan_distance(g1: Galaxy, g2: Galaxy) -> u64 {
        let x_diff_abs = (g2.x as i64 - g1.x as i64).abs() as u64;
        let y_diff_abs = (g2.y as i64 - g1.y as i64).abs() as u64;
        x_diff_abs + y_diff_abs
    }
}

struct GalaxyCombinations<'a> {
    galaxies: &'a [Galaxy],
    i: usize,
    j: usize,
}

impl<'a> Iterator for GalaxyCombinations<'a> {
    type Item = (Galaxy, Galaxy);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.j == self.galaxies.len() {
                self.i += 1;
                self.j = 0;
            }
            if self.i == self.galaxies.len() {
                return None;
            }

            let i = self.i;
            let j = self.j;
            self.j += 1;
            match j > i {
                true => return Some((self.galaxies[i], self.galaxies[j])),
                false => continue,
            }
        }
    }
}
