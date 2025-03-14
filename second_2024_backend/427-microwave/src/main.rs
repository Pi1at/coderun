use std::collections::BTreeSet;
use std::io::BufRead;

fn solve(n: u64, [coefs @ .., m]: &[u64; 3]) -> u64 {
    //let m = coefs.last().unwrap();
    let mut dist = vec![-1_i64; *m as usize];
    let mut q = BTreeSet::new();
    dist[0] = 0;
    q.insert(0);
    let mut answer = 0;

    while let Some(v) = q.pop_first() {
        if v >= n {
            continue;
        }
        let d = (n - 1) - v;
        answer += 1 + d / *m;

        for &coef in &coefs[0..2] {
            let next_v = v + coef;
            let mut residue = next_v % *m;
            // if residue < 0 {
            //     residue += *m;
            // }
            let residue = residue as usize;

            if dist[residue] == -1 || (next_v as i64) < dist[residue] {
                if dist[residue] != -1 {
                    q.remove(&(dist[residue] as u64));
                }
                dist[residue] = next_v as i64;
                q.insert(next_v);
            }
        }
    }
    answer
}

fn main() {
    let mut lines = std::io::stdin().lock().lines().map_while(Result::ok);
    let n: u64 = lines.next().unwrap().parse().unwrap();
    let mut coefs =
        lines.next().unwrap().split_ascii_whitespace().flat_map(str::parse).collect::<Vec<u64>>();
    drop(lines);
    coefs.sort_unstable();

    let answer = solve(n, &coefs[0..3]);
    println!("{answer}");
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
    fn test_10() {
        assert_eq!(8, solve(10, &[3, 4, 5]));
    }
}

// endregion: --- Tests
