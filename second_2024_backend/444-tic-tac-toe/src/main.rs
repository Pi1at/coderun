use std::io::{self, BufWriter, Write};

use lib::{read_input, read_pair};

use domain::{Counter, Field, ADIAG, DIAG, HORIZ, VERT};

pub mod domain {
    use std::{convert::Infallible, str::FromStr};

    #[derive(Eq, PartialEq, Hash, Debug, Default, Clone)]
    pub enum Field {
        #[default]
        Empty,
        Crosses,
        Noughts,
    }

    impl FromStr for Field {
        type Err = Infallible;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "X" => Ok(Self::Crosses),
                "O" => Ok(Self::Noughts),
                "." => Ok(Self::Empty),
                _ => Ok(Self::default()), // Orly
            }
        }
    }

    impl From<u8> for Field {
        fn from(value: u8) -> Self {
            match value {
                b'X' => Self::Crosses,
                b'O' => Self::Noughts,
                b'.' => Self::Empty,
                _ => unreachable!(), // Orly
            }
        }
    }
    impl From<&u8> for Field {
        fn from(value: &u8) -> Self {
            match value {
                b'X' => Self::Crosses,
                b'O' => Self::Noughts,
                b'.' => Self::Empty,
                _ => unreachable!(), // Orly
            }
        }
    }
    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Copy, Clone)]
    pub struct Counter {
        pub h: u8, // horizontal
        pub v: u8, // vertical
        pub d: u8, // diagonal
        pub r: u8, // reverse diagonal
    }
    impl Counter {
        #[must_use]
        pub fn new() -> Self {
            Self::default()
        }
        #[must_use]
        pub fn max_v(&self) -> u8 {
            self.h.max(self.v).max(self.d.max(self.r))
        }
    }
    pub const HORIZ: [(isize, isize); 4] = [(1, 0), (2, 0), (3, 0), (4, 0)];
    pub const VERT: [(isize, isize); 4] = [(0, 1), (0, 2), (0, 3), (0, 4)];
    pub const DIAG: [(isize, isize); 4] = [(1, 1), (2, 2), (3, 3), (4, 4)];
    pub const ADIAG: [(isize, isize); 4] = [(-1, 1), (-2, 2), (-3, 3), (-4, 4)];
}

// TODO: refactor this mess
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = read_input()?;
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());

    let (irows, icols) = read_pair::<isize, isize>(&lines.next().unwrap());
    let (rows, cols) = (irows as usize, icols as usize);
    let is_valid_coord =
        |(col, row): (isize, isize)| (0..=icols).contains(&col) && (0..=irows).contains(&row);

    let mut is_win = false;
    let mut crosses_board = vec![vec![Counter::default(); cols]; rows];
    let mut nougths_board = vec![vec![Counter::default(); cols]; rows];

    for row in 0..irows {
        for (col, v) in (0..icols).zip(lines.next().unwrap().bytes().map(Field::from)) {
            match v {
                Field::Crosses => {
                    if crosses_board[row as usize][col as usize].max_v() == 4 {
                        is_win = true;
                        break;
                    };
                    HORIZ
                        .iter()
                        .map(|(x, y)| (x + col, y + row))
                        .filter_map(|(x, y)| {
                            is_valid_coord((x, y)).then_some((x as usize, y as usize))
                        })
                        .for_each(|(x, y)| crosses_board[y][x].h += 1);
                    VERT.iter()
                        .map(|(x, y)| (x + col, y + row))
                        .filter_map(|(x, y)| {
                            is_valid_coord((x, y)).then_some((x as usize, y as usize))
                        })
                        .for_each(|(x, y)| crosses_board[y][x].v += 1);
                    DIAG.iter()
                        .map(|(x, y)| (x + col, y + row))
                        .filter_map(|(x, y)| {
                            is_valid_coord((x, y)).then_some((x as usize, y as usize))
                        })
                        .for_each(|(x, y)| crosses_board[y][x].d += 1);
                    ADIAG
                        .iter()
                        .map(|(x, y)| (x + col, y + row))
                        .filter_map(|(x, y)| {
                            is_valid_coord((x, y)).then_some((x as usize, y as usize))
                        })
                        .for_each(|(x, y)| crosses_board[y][x].r += 1);
                }
                Field::Noughts => {
                    if nougths_board[row as usize][col as usize].max_v() == 4 {
                        is_win = true;
                        break;
                    };
                    HORIZ
                        .iter()
                        .map(|(x, y)| (x + col, y + row))
                        .filter_map(|(x, y)| {
                            is_valid_coord((x, y)).then_some((x as usize, y as usize))
                        })
                        .for_each(|(x, y)| nougths_board[y][x].h += 1);
                    VERT.iter()
                        .map(|(x, y)| (x + col, y + row))
                        .filter_map(|(x, y)| {
                            is_valid_coord((x, y)).then_some((x as usize, y as usize))
                        })
                        .for_each(|(x, y)| nougths_board[y][x].v += 1);
                    DIAG.iter()
                        .map(|(x, y)| (x + col, y + row))
                        .filter_map(|(x, y)| {
                            is_valid_coord((x, y)).then_some((x as usize, y as usize))
                        })
                        .for_each(|(x, y)| nougths_board[y][x].d += 1);
                    ADIAG
                        .iter()
                        .map(|(x, y)| (x + col, y + row))
                        .filter_map(|(x, y)| {
                            is_valid_coord((x, y)).then_some((x as usize, y as usize))
                        })
                        .for_each(|(x, y)| nougths_board[y][x].r += 1);
                }
                Field::Empty => {}
            };
        }
        if is_win {
            break;
        }
    }
    writeln!(out, "{}", if is_win { "Yes" } else { "No" })?;
    Ok(())
}

// region:    --- Tests

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;
}

// endregion: --- Tests

// region: --- Lib
pub mod lib {
    use std::{
        env,
        fmt::Display,
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
        str::FromStr,
    };

    pub fn read_pair<T1, T2>(s: &str) -> (T1, T2)
    where
        T1: FromStr + Copy,
        T2: FromStr + Copy,
    {
        let mut iter = s.split_ascii_whitespace();
        match (iter.next().map(str::parse), iter.next().map(str::parse)) {
            (Some(Ok(first)), Some(Ok(snd))) => (first, snd),
            _ => unreachable!("input is malformed!"),
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn read_input() -> Result<impl Iterator<Item = String>, Box<dyn std::error::Error>> {
        let local_mode = env::var("CODERUN_LOCAL").is_ok_and(|m| m == "true");
        let path = {
            let mut path = PathBuf::new();
            if local_mode {
                path.push(env::var("CARGO_MANIFEST_DIR")?);
            }
            path.push("input.txt");
            path
        };
        let file = File::open(path)?;
        Ok(BufReader::new(file).lines().map_while(Result::ok))
    }

    pub fn join_into_string<T>(a: &[T]) -> String
    where
        T: ToString,
    {
        let Some((first, suffix)) = a.split_first() else { return String::new() };
        let first_owned = first.to_string();
        suffix.iter().fold(first_owned, |mut a, b| {
            a.push(' ');
            a.push_str(&b.to_string());
            a
        })
    }

    pub trait VecStuff<T> {
        fn print(&self, w: usize);
    }
    impl<T: Display + Clone + Copy + 'static> VecStuff<T> for &[T] {
        fn print(&self, w: usize) {
            self.iter().enumerate().take(w).for_each(|(i, e)| {
                if i != 0 {
                    print!(" ");
                }
                print!("{e:5}");
            });
        }
    }
}
// endregion" --- Lib
