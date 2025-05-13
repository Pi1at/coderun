// TODO: reimplement
use std::io::{self, BufWriter, Write};

use lib::read_input;

const fn solve(players: &[u16]) -> usize {
    let mut queens_left = 4;
    let mut idx = 0;
    while queens_left > 0 && idx < players.len() {
        if players[idx] <= queens_left {
            queens_left -= players[idx];
            idx += 1;
        } else {
            break;
        }
    }
    // idx == 0, all lies 4
    // idx == 1, three lie 3
    // idx == 2, two lies 2
    // idx == 3, one lies 1
    // idx == 4, none if queens left == 0
    if queens_left > 2 && idx >= players.len() {
        1
    } else {
        4 - idx
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = read_input()?;
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());

    let mut players_lies =
        lines.next().unwrap().split_ascii_whitespace().flat_map(str::parse).collect::<Vec<u16>>();
    players_lies.sort_unstable();
    writeln!(out, "{}", solve(&players_lies))?;
    Ok(())
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use lib::join_into_string;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_all_cases() {
        let mut res = Vec::<Vec<u16>>::new();
        for a in 0..=4 {
            for b in a..=4 {
                for c in b..=4 {
                    for d in c..=4 {
                        res.push(vec![a, b, c, d]);
                    }
                }
            }
        }
        println!("{}", res.len());
        for p in res {
            println!("{} - {}", join_into_string(&p), solve(&p));
        }
    }
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
