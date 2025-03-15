use lib::{read_input, read_pair};
use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = read_input()?;
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());
    let (black, white) = read_pair::<u32, u32>(&lines.next().unwrap());
    assert!(white >= 1);
    // there is no sense when b < 8
    assert!(black >= 8);
    // or maybe better just use binsearch?
    // n*m == B+W , min_n = max_m => min_n = (B+W).sqrt(2)
    // 2*(n+m) = B+4
    // 2*n + 2* max_m = B+4 => n = B/2 + 2 - (B+w).sqrt(2)
    // W = 1, min m = 3 => max_n = B - 2
    // 2*(n+3) = B + 4 => n = B/2 - 1
    // => m = (B+4-2*n)/2
    // let x = n, find bigger value
    // x*(B+4-2*x)/2 - B - W = 0
    // -x^2 + x*(B+4)/2 - B - W = 0
    // x^2 - x * (B+4)/2 + B + W = 0
    // D = ((B/2+2)^2 - 4 *(B+w)).sqrt()

    let b_coeff = -(f64::from(black) + 4.0) / 2.0;
    let c_coeff = f64::from(black + white);
    let discr = 4.0_f64.mul_add(-c_coeff, b_coeff.powi(2));
    let n = 0.5 * (-b_coeff + discr.sqrt());
    let m = 0.5 * (-b_coeff - discr.sqrt());
    writeln!(out, "{n} {m}")?;
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
use lib::{read_input, read_pair};
use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = read_input()?;
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());
    let (black, white) = read_pair::<u32, u32>(&lines.next().unwrap());
    assert!(white >= 1);
    // there is no sense when b < 8
    assert!(black >= 8);
    // or maybe better just use binsearch?
    // n*m == B+W , min_n = max_m => min_n = (B+W).sqrt(2)
    // 2*(n+m) = B+4
    // 2*n + 2* max_m = B+4 => n = B/2 + 2 - (B+w).sqrt(2)
    // W = 1, min m = 3 => max_n = B - 2
    // 2*(n+3) = B + 4 => n = B/2 - 1
    // => m = (B+4-2*n)/2
    // let x = n, find bigger value
    // x*(B+4-2*x)/2 - B - W = 0
    // -x^2 + x*(B+4)/2 - B - W = 0
    // x^2 - x * (B+4)/2 + B + W = 0
    // D = ((B/2+2)^2 - 4 *(B+w)).sqrt()

    let b_coeff = -(f64::from(black) + 4.0) / 2.0;
    let c_coeff = f64::from(black + white);
    let discr = 4.0_f64.mul_add(-c_coeff, b_coeff.powi(2));
    let n = 0.5 * (-b_coeff + discr.sqrt());
    let m = 0.5 * (-b_coeff - discr.sqrt());
    writeln!(out, "{n} {m}")?;
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
