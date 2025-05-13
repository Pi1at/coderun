use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::BufRead;

fn solve(n: usize, c: &[u32]) -> usize {
    // n <= 10^18
    const EMPTY: usize = 10_usize.pow(18);
    match c {
        [coefs @ .., m] => {
            let m = *m as usize;
            let mut dist = vec![EMPTY; m];
            dist[0] = 0;

            let mut queue = BinaryHeap::new();
            queue.push(Reverse(0));

            let mut answer = 0;

            // get minimum element
            while let Some(Reverse(v)) = queue.pop() {
                if queue.peek().is_some_and(|Reverse(next_v)| *next_v == v) {
                    continue;
                }
                let d = (n - 1) - v;
                answer += 1 + d / m;

                for &coef in coefs {
                    let next_v = v + (coef as usize);
                    if next_v >= n {
                        continue;
                    }
                    let residue = next_v % m;
                    if dist[residue] == EMPTY {
                        dist[residue] = next_v;
                        queue.push(Reverse(next_v));
                    }
                }
            }
            answer
        },
        [] => 1, //cover 0 case
    }
}

fn main() {
    let mut lines = std::io::stdin().lock().lines().map_while(Result::ok);
    let n = lines.next().unwrap().parse().unwrap();
    let mut coefs = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .flat_map(str::parse)
        .filter(|&v| (v as usize) < n)
        .collect::<Vec<u32>>();
    drop(lines);
    coefs.sort_unstable();
    println!("{}", solve(n, &coefs));
}

// region:    --- Tests

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_big() {
        assert_eq!(
            999_999_998_333_383_333,
            solve(1_000_000_000_000_000_000, &[100_000, 99999, 99997])
        );
    }

    #[test]
    fn test_100() {
        assert_eq!(85, solve(100, &[6, 10, 15]));
    }

    #[test]
    fn test_20() {
        assert_eq!(5, solve(20, &[6, 14, 34]));
    }

    #[test]
    fn test_20_() {
        assert_eq!(5, solve(20, &[6, 14]));
    }

    #[test]
    fn test_5() {
        assert_eq!(5, solve(5, &[1]));
    }

    #[test]
    fn test_10() {
        assert_eq!(8, solve(10, &[3, 4, 5]));
    }
}

// endregion: --- Tests
