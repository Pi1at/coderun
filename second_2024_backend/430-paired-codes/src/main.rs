use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn solve(codes: &str) -> usize {
    // store counters for each bitset
    let mut table = codes
        .split_ascii_whitespace()
        .map(|s| s.bytes().fold(0_u16, |acc, v| acc | (1 << (v - b'0'))))
        .fold(HashMap::new(), |mut acc, key| {
            acc.entry(key).and_modify(|c| *c += 1).or_insert(1);
            acc
        })
        .into_iter()
        .collect::<Vec<_>>();
    let mut total_pairs_count = 0_usize;
    while let Some((code_1, count_1)) = table.pop() {
        let this_pairs_count =
            table.iter().fold(0_usize, |mut cur_pair_count, (code_2, count_2)| {
                if code_1 & code_2 != 0 {
                    cur_pair_count += count_1 * count_2;
                };
                cur_pair_count
            });
        // account for pair of same code
        total_pairs_count += this_pairs_count + count_1 * (count_1 - 1) / 2;
    }
    total_pairs_count
}

fn main() {
    let codes = io::stdin().lock().lines().map_while(Result::ok).nth(1).unwrap();
    let ans = solve(&codes);
    println!("{ans}");
}

#[cfg(test)]
mod tests {

    use super::*;

    fn solve_slow(codes: &str) -> usize {
        codes
            .split_ascii_whitespace()
            .map(|s| s.bytes().fold(0_u16, |acc, v| acc | (1 << (v - b'0'))))
            .fold((HashMap::new(), 0_usize), |(mut acc, mut pairs), key| {
                let this_num_count = acc.iter().fold(0_u32, |mut cur_pair_count, (k, v)| {
                    if key & k != 0 {
                        cur_pair_count += v;
                    }
                    cur_pair_count
                });
                pairs += this_num_count as usize;
                acc.entry(key).and_modify(|c| *c += 1).or_insert(1);
                (acc, pairs)
            })
            .1
    }

    #[test]
    fn test_1() {
        assert_eq!(solve("103 123 20 4567"), 3);
    }

    #[test]
    fn test_2() {
        //just some counting
        use std::fmt::Write;
        let x = (1_000_000_usize..=1_023_456_usize).fold(String::new(), |mut acc, v| {
            let _ = write!(&mut acc, "{v} ");
            acc
        });
        dbg!("!");
        //let ok_c = solve_slow(&x);
        dbg!("!");
        let m_c = solve(&x);
        assert_eq!(275_103_696, m_c);
    }
    #[test]
    fn generate_proper_ans() {
        //just some counting
        use std::fmt::Write;
        let x = (1_000_000_usize..=1_023_456_usize).fold(String::new(), |mut acc, v| {
            let _ = write!(&mut acc, "{v} ");
            acc
        });
        let ok_c = solve_slow(&x);
        assert_eq!(275_103_696, ok_c);
    }
}
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn solve(codes: &str) -> usize {
    // store counters for each bitset
    let mut table = codes
        .split_ascii_whitespace()
        .map(|s| s.bytes().fold(0_u16, |acc, v| acc | (1 << (v - b'0'))))
        .fold(HashMap::new(), |mut acc, key| {
            acc.entry(key).and_modify(|c| *c += 1).or_insert(1);
            acc
        })
        .into_iter()
        .collect::<Vec<_>>();
    let mut total_pairs_count = 0_usize;
    while let Some((code_1, count_1)) = table.pop() {
        let this_pairs_count =
            table.iter().fold(0_usize, |mut cur_pair_count, (code_2, count_2)| {
                if code_1 & code_2 != 0 {
                    cur_pair_count += count_1 * count_2;
                };
                cur_pair_count
            });
        // account for pair of same code
        total_pairs_count += this_pairs_count + count_1 * (count_1 - 1) / 2;
    }
    total_pairs_count
}

fn main() {
    let codes = io::stdin().lock().lines().map_while(Result::ok).nth(1).unwrap();
    let ans = solve(&codes);
    println!("{ans}");
}

#[cfg(test)]
mod tests {

    use super::*;

    fn solve_slow(codes: &str) -> usize {
        codes
            .split_ascii_whitespace()
            .map(|s| s.bytes().fold(0_u16, |acc, v| acc | (1 << (v - b'0'))))
            .fold((HashMap::new(), 0_usize), |(mut acc, mut pairs), key| {
                let this_num_count = acc.iter().fold(0_u32, |mut cur_pair_count, (k, v)| {
                    if key & k != 0 {
                        cur_pair_count += v;
                    }
                    cur_pair_count
                });
                pairs += this_num_count as usize;
                acc.entry(key).and_modify(|c| *c += 1).or_insert(1);
                (acc, pairs)
            })
            .1
    }

    #[test]
    fn test_1() {
        assert_eq!(solve("103 123 20 4567"), 3);
    }

    #[test]
    fn test_2() {
        //just some counting
        use std::fmt::Write;
        let x = (1_000_000_usize..=1_023_456_usize).fold(String::new(), |mut acc, v| {
            let _ = write!(&mut acc, "{v} ");
            acc
        });
        dbg!("!");
        //let ok_c = solve_slow(&x);
        dbg!("!");
        let m_c = solve(&x);
        assert_eq!(275_103_696, m_c);
    }
    #[test]
    fn generate_proper_ans() {
        //just some counting
        use std::fmt::Write;
        let x = (1_000_000_usize..=1_023_456_usize).fold(String::new(), |mut acc, v| {
            let _ = write!(&mut acc, "{v} ");
            acc
        });
        let ok_c = solve_slow(&x);
        assert_eq!(275_103_696, ok_c);
    }
}
