use std::{
    io::{self, BufWriter, Write},
    str::FromStr,
};

// TODO: implement lazy part

use lib::read_input;
use st::SegmentTree;

mod st {
    use std::ops::Range;

    const MODULO: u64 = 1_000_000_007;
    // range modify with increment
    // range query mul % MODULO
    #[allow(dead_code)]
    pub struct SegmentTree {
        data: Vec<u32>,
        store: Vec<Option<u32>>,
        n: usize,
        height: usize,
    }

    impl SegmentTree {
        pub fn build(init: &[u32]) -> Self {
            let n = init.len();
            // height of the tree
            let height = (usize::BITS - n.leading_zeros()) as usize;
            // identify for query op
            let mut data = vec![1; 2 * n];
            let store = vec![None; n];
            data[n..2 * n].copy_from_slice(init);
            let mut t = Self { data, store, n, height };
            t.update_nodes();
            t
        }

        pub fn update_nodes(&mut self) {
            for i in (1..self.n).rev() {
                self.data[i] = Self::combine(self.data[i << 1], self.data[i << 1 | 1]);
            }
        }

        pub fn modify_by_one(&mut self, p: usize) {
            let mut p = p + self.n;
            // store value to leaf
            self.data[p] += 1;

            while p > 1 {
                // update parent node
                self.data[p >> 1] = Self::combine(self.data[p], self.data[p ^ 1]);
                // go up
                p >>= 1;
            }
        }

        // increment values in range [l..r]
        pub fn modify_range(&mut self, range: Range<usize>) {
            for idx in range {
                self.modify_by_one(idx);
            }
        }

        // return result of product on range [l..r] % MODULO
        pub fn query_range(&self, range: Range<usize>) -> u32 {
            // for multiplication identify is 1
            let mut res = 1;
            let mut l = range.start + self.n;
            let mut r = range.end + self.n;
            // If l, the left interval border, is odd (which is equivalent to l&1) then l is the right child of its parent
            // Then our interval includes node l but doesn't include it's parent. So we add data[l] and move to the right
            // of l's parent by setting l = (l + 1) / 2. If l is even, it is the left child, and the interval includes its parent as well
            // (unless the right border interferes), so we just move to it by setting l = l / 2.
            // Similar argumentation is applied to the right border. We stop once borders meet.
            while l < r {
                if l & 1 != 0 {
                    res = Self::combine(res, self.data[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    res = Self::combine(res, self.data[r]);
                }
                l >>= 1;
                r >>= 1;
            }

            res
        }

        // calculates query value
        #[allow(clippy::cast_possible_truncation)]
        const fn combine(a: u32, b: u32) -> u32 {
            (((a as u64) * (b as u64)) % MODULO) as u32
        }
    }
}

enum BallQuery {
    Put { l: usize, r: usize },
    Count { l: usize, r: usize },
}

impl FromStr for BallQuery {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split_ascii_whitespace().take(3).flat_map(str::parse);
        match (nums.next(), nums.next(), nums.next()) {
            (Some(t), Some(l), Some(r)) => {
                if t == 0 {
                    Ok(Self::Put { l, r })
                } else {
                    Ok(Self::Count { l, r })
                }
            }
            _ => unreachable!("input malformed!"),
        }
    }
}

fn solve_st<I, S>(init: &[u32], mut queries: I) -> impl Iterator<Item = usize>
where
    S: AsRef<str>,
    I: Iterator<Item = S>,
{
    let mut rmq = SegmentTree::build(init);
    std::iter::from_fn(move || {
        let mut v = queries.next().and_then(|s| s.as_ref().parse::<BallQuery>().ok());
        //updating counters
        while let Some(BallQuery::Put { l, r }) = v {
            rmq.modify_range(l - 1..r);
            v = queries.next().and_then(|s| s.as_ref().parse::<BallQuery>().ok());
        }
        if let Some(BallQuery::Count { l, r }) = v {
            Some(rmq.query_range(l - 1..r) as usize)
        } else {
            None
        }
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = read_input()?;
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());
    let initial_baskets =
        lines.nth(1).unwrap().split_ascii_whitespace().flat_map(str::parse).collect::<Vec<_>>();
    let queries = lines.skip(1);
    for res in solve_st(&initial_baskets, queries) {
        writeln!(out, "{res}")?;
    }
    Ok(())
}

// region:    --- Tests

#[cfg(test)]
mod tests {

    use super::*;

    fn solve_naive<I, S>(init: &[u32], mut queries: I) -> impl Iterator<Item = u32>
    where
        S: AsRef<str>,
        I: Iterator<Item = S>,
    {
        const MODULO: u32 = 1_000_000_007;
        let mut b = Vec::from(init);
        std::iter::from_fn(move || {
            let mut v = queries.next().and_then(|s| s.as_ref().parse::<BallQuery>().ok());
            //updating counters
            while let Some(BallQuery::Put { l, r }) = v {
                (l..=r).for_each(|idx| {
                    b[idx - 1] += 1;
                });
                v = queries.next().and_then(|s| s.as_ref().parse::<BallQuery>().ok());
            }
            if let Some(BallQuery::Count { l, r }) = v {
                let mut count = 1;
                (l..=r).for_each(|idx| {
                    count *= b[idx - 1];
                    if count > MODULO {
                        count %= MODULO;
                    }
                });
                Some(count)
            } else {
                None
            }
        })
    }

    #[test]
    fn test_1() {
        assert_eq!(
            solve_naive(&[1, 2, 3], ["1 1 3", "0 1 2", "1 1 2"].into_iter()).collect::<Vec<_>>(),
            [6, 6]
        );
    }
    #[test]
    fn test_1_st() {
        assert_eq!(
            solve_st(&[1, 2, 3], ["1 1 3", "0 1 2", "1 1 2"].into_iter()).collect::<Vec<_>>(),
            [6, 6]
        );
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
