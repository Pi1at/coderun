use std::io::{self, BufWriter, Write};

use lib::{read_input, read_pair};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = read_input()?;
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());

    let (n, m) = read_pair::<usize, usize>(&lines.next().unwrap());
    let mut garden: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        let row: Vec<usize> =
            lines.next().unwrap().split_whitespace().flat_map(str::parse).collect();
        garden.push(row);
    }
    drop(lines);

    let mut dp: Vec<Vec<usize>> = vec![vec![0; m]; n];
    let mut max_side = 0;
    let mut max_i = 0;
    let mut max_j = 0;

    for i in 0..n {
        for j in 0..m {
            if garden[i][j] == 1 {
                if i == 0 || j == 0 {
                    dp[i][j] = 1;
                } else {
                    dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1].min(dp[i - 1][j - 1]));
                }

                if dp[i][j] >= max_side {
                    max_side = dp[i][j];
                    max_i = i + 1 - max_side + 1;
                    max_j = j + 1 - max_side + 1;
                }
            }
        }
    }

    writeln!(out, "{max_side}\n{max_i} {max_j}")?;
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
