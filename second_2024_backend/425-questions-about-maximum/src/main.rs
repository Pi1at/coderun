use std::io::{self, BufWriter, Write};
use std::iter;

use domain::{Action, Query};
use lib::{join_into_string, read_input, read_pair};

pub mod domain {
    use std::fmt::Display;
    use std::str::FromStr;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Query {
        pub l: u16,
        pub r: u16,
        pub x: u16,
    }

    impl Display for Query {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {} {}", self.l, self.r, self.x)
        }
    }

    impl FromStr for Query {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut iter = s.split_ascii_whitespace().flat_map(str::parse);
            match (iter.next(), iter.next(), iter.next()) {
                (Some(l), Some(r), Some(x)) => Ok(Self { l, r, x }),
                _ => Err("input is malformed!"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Action {
        Add(u16),
        Remove(u16),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = read_input()?;
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());

    // n - array size
    // q - number of queries
    let (n, q): (usize, usize) = read_pair(&input.next().unwrap());

    // transform queries to actions, also modify to start from zero

    let actions = {
        let mut actions = Vec::with_capacity(q * 2);
        input.flat_map(|s| s.parse::<Query>()).for_each(|q| {
            actions.push((q.l - 1, Action::Add(q.x)));
            actions.push((q.r - 1, Action::Remove(q.x)));
        });

        // sort by index, then by type of operation
        actions.sort_unstable();
        actions
    };

    // storing bag
    let mut bag = vec![0_u64; n + 1];
    bag[0] = 1;

    let mut possible = vec![false; n + 1];

    for (_, event) in actions {
        match event {
            Action::Add(v) => {
                // add v to the bag
                for i in (v as usize..=n).rev() {
                    bag[i] += bag[i - v as usize];
                }
                iter::zip(bag[1..].iter(), possible[1..].iter_mut()).for_each(|(b, p)| {
                    *p = *p || *b > 0;
                });
            },
            Action::Remove(v) => {
                // remove v
                for i in v as usize..=n {
                    bag[i] -= bag[i - v as usize];
                }
            },
        }
    }
    // n <= 10^4
    let answer: Vec<u16> = (1..).take(n).filter(|&i| possible[i as usize]).collect();
    writeln!(out, "{}", answer.len())?;
    writeln!(out, "{}", join_into_string(&answer))?;
    Ok(())
}

// region: --- Lib
pub mod lib {
    use std::env;
    use std::fmt::Display;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;
    use std::str::FromStr;

    #[allow(clippy::missing_errors_doc)]
    pub fn read_input() -> Result<impl Iterator<Item = String>, Box<dyn std::error::Error>> {
        let local_mode = env::var("USER")
            .or_else(|_| env::var("USERNAME"))
            .is_ok_and(|u| u.to_lowercase() == "pi1at");
        //let local_mode = env::var("CODERUN_LOCAL").is_ok_and(|m| m == "true");
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
    pub fn join_into_string<T>(a: &[T]) -> String
    where T: ToString {
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
    impl<T: Display + Clone + Copy + 'static> VecStuff<T> for Vec<T> {
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
use std::io::{self, BufWriter, Write};
use std::iter;

use domain::{Action, Query};
use lib::{join_into_string, read_input, read_pair};

pub mod domain {
    use std::fmt::Display;
    use std::str::FromStr;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Query {
        pub l: u16,
        pub r: u16,
        pub x: u16,
    }

    impl Display for Query {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {} {}", self.l, self.r, self.x)
        }
    }

    impl FromStr for Query {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut iter = s.split_ascii_whitespace().flat_map(str::parse);
            match (iter.next(), iter.next(), iter.next()) {
                (Some(l), Some(r), Some(x)) => Ok(Self { l, r, x }),
                _ => Err("input is malformed!"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Action {
        Add(u16),
        Remove(u16),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = read_input()?;
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());

    // n - array size
    // q - number of queries
    let (n, q): (usize, usize) = read_pair(&input.next().unwrap());

    // transform queries to actions, also modify to start from zero

    let actions = {
        let mut actions = Vec::with_capacity(q * 2);
        input.flat_map(|s| s.parse::<Query>()).for_each(|q| {
            actions.push((q.l - 1, Action::Add(q.x)));
            actions.push((q.r - 1, Action::Remove(q.x)));
        });

        // sort by index, then by type of operation
        actions.sort_unstable();
        actions
    };

    // storing bag
    let mut bag = vec![0_u64; n + 1];
    bag[0] = 1;

    let mut possible = vec![false; n + 1];

    for (_, event) in actions {
        match event {
            Action::Add(v) => {
                // add v to the bag
                for i in (v as usize..=n).rev() {
                    bag[i] += bag[i - v as usize];
                }
                iter::zip(bag[1..].iter(), possible[1..].iter_mut()).for_each(|(b, p)| {
                    *p = *p || *b > 0;
                });
            },
            Action::Remove(v) => {
                // remove v
                for i in v as usize..=n {
                    bag[i] -= bag[i - v as usize];
                }
            },
        }
    }
    // n <= 10^4
    let answer: Vec<u16> = (1..).take(n).filter(|&i| possible[i as usize]).collect();
    writeln!(out, "{}", answer.len())?;
    writeln!(out, "{}", join_into_string(&answer))?;
    Ok(())
}

// region: --- Lib
pub mod lib {
    use std::env;
    use std::fmt::Display;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;
    use std::str::FromStr;

    #[allow(clippy::missing_errors_doc)]
    pub fn read_input() -> Result<impl Iterator<Item = String>, Box<dyn std::error::Error>> {
        let local_mode = env::var("USER")
            .or_else(|_| env::var("USERNAME"))
            .is_ok_and(|u| u.to_lowercase() == "pi1at");
        //let local_mode = env::var("CODERUN_LOCAL").is_ok_and(|m| m == "true");
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
    pub fn join_into_string<T>(a: &[T]) -> String
    where T: ToString {
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
    impl<T: Display + Clone + Copy + 'static> VecStuff<T> for Vec<T> {
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
