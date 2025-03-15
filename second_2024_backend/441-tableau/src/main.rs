use std::{
    collections::HashMap,
    io::{self, BufWriter, Write},
};

use domain::{PlayerScore, ScoreTable};
use lib::read_input;

pub mod domain {
    use std::{fmt::Display, str::FromStr};

    #[derive(Debug, PartialEq, Eq)]
    pub struct PlayerScore {
        pub name: String,
        pub score: u16,
    }

    impl PartialOrd for PlayerScore {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for PlayerScore {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match self.score.cmp(&other.score) {
                std::cmp::Ordering::Equal => self.name.cmp(&other.name),
                v => v,
            }
        }
    }

    impl From<(String, u16)> for PlayerScore {
        fn from((name, score): (String, u16)) -> Self {
            Self { name, score }
        }
    }
    impl Display for PlayerScore {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {}", self.name, self.score)
        }
    }

    impl FromStr for PlayerScore {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut iter = s.split_ascii_whitespace();
            match (iter.next().map(str::parse), iter.next().map(str::parse)) {
                (Some(Ok(name)), Some(Ok(score))) => Ok(Self { name, score }),
                (Some(Ok(name)), None) => Ok(Self { name, score: Default::default() }),
                _ => Err("input is malformed!"),
            }
        }
    }
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
    pub struct ScoreTable {
        pub left: u16,
        pub right: u16,
    }

    impl ScoreTable {
        #[must_use]
        pub const fn score_diff(&self, other: &Self) -> u16 {
            self.left.abs_diff(other.left) + self.right.abs_diff(other.right)
        }
    }

    impl Display for ScoreTable {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}:{}", self.left, self.right)
        }
    }

    impl FromStr for ScoreTable {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut iter = s.split(':');
            match (iter.next().map(str::parse), iter.next().map(str::parse)) {
                (Some(Ok(left)), Some(Ok(right))) => Ok(Self { left, right }),
                _ => Err("input is malformed!"),
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = read_input()?;
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());

    let num_players: usize = lines.next().unwrap().parse()?;
    let players =
        lines.by_ref().take(num_players).map(|s| (s, 0)).collect::<HashMap<String, u16>>();
    let num_scores: usize = lines.next().unwrap().parse()?;
    let mut player_table = lines
        .take(num_scores)
        .map(|s| {
            let mut iter = s.split_ascii_whitespace();
            let score: ScoreTable = iter.next().unwrap().parse().unwrap();
            let player = iter.next().unwrap().to_owned();
            (score, player)
        })
        .fold((players, ScoreTable::default()), |(mut acc, prev_score), (score, player)| {
            acc.entry(player).and_modify(|v| {
                *v += score.score_diff(&prev_score);
            });
            (acc, score)
        })
        .0
        .into_iter()
        .map(PlayerScore::from)
        .collect::<Vec<PlayerScore>>();
    // TODO: just take max?
    player_table.sort_unstable();
    writeln!(out, "{}", player_table.last().unwrap())?;
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
use std::{
    collections::HashMap,
    io::{self, BufWriter, Write},
};

use domain::{PlayerScore, ScoreTable};
use lib::read_input;

pub mod domain {
    use std::{fmt::Display, str::FromStr};

    #[derive(Debug, PartialEq, Eq)]
    pub struct PlayerScore {
        pub name: String,
        pub score: u16,
    }

    impl PartialOrd for PlayerScore {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for PlayerScore {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match self.score.cmp(&other.score) {
                std::cmp::Ordering::Equal => self.name.cmp(&other.name),
                v => v,
            }
        }
    }

    impl From<(String, u16)> for PlayerScore {
        fn from((name, score): (String, u16)) -> Self {
            Self { name, score }
        }
    }
    impl Display for PlayerScore {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {}", self.name, self.score)
        }
    }

    impl FromStr for PlayerScore {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut iter = s.split_ascii_whitespace();
            match (iter.next().map(str::parse), iter.next().map(str::parse)) {
                (Some(Ok(name)), Some(Ok(score))) => Ok(Self { name, score }),
                (Some(Ok(name)), None) => Ok(Self { name, score: Default::default() }),
                _ => Err("input is malformed!"),
            }
        }
    }
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
    pub struct ScoreTable {
        pub left: u16,
        pub right: u16,
    }

    impl ScoreTable {
        #[must_use]
        pub const fn score_diff(&self, other: &Self) -> u16 {
            self.left.abs_diff(other.left) + self.right.abs_diff(other.right)
        }
    }

    impl Display for ScoreTable {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}:{}", self.left, self.right)
        }
    }

    impl FromStr for ScoreTable {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut iter = s.split(':');
            match (iter.next().map(str::parse), iter.next().map(str::parse)) {
                (Some(Ok(left)), Some(Ok(right))) => Ok(Self { left, right }),
                _ => Err("input is malformed!"),
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = read_input()?;
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());

    let num_players: usize = lines.next().unwrap().parse()?;
    let players =
        lines.by_ref().take(num_players).map(|s| (s, 0)).collect::<HashMap<String, u16>>();
    let num_scores: usize = lines.next().unwrap().parse()?;
    let mut player_table = lines
        .take(num_scores)
        .map(|s| {
            let mut iter = s.split_ascii_whitespace();
            let score: ScoreTable = iter.next().unwrap().parse().unwrap();
            let player = iter.next().unwrap().to_owned();
            (score, player)
        })
        .fold((players, ScoreTable::default()), |(mut acc, prev_score), (score, player)| {
            acc.entry(player).and_modify(|v| {
                *v += score.score_diff(&prev_score);
            });
            (acc, score)
        })
        .0
        .into_iter()
        .map(PlayerScore::from)
        .collect::<Vec<PlayerScore>>();
    // TODO: just take max?
    player_table.sort_unstable();
    writeln!(out, "{}", player_table.last().unwrap())?;
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
