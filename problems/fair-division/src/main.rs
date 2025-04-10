use std::{cmp::Reverse, collections::BTreeSet, io::BufRead};

#[must_use]
pub const fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    let tz_a = a.trailing_zeros();
    let tz_b = b.trailing_zeros();
    a >>= tz_a;
    b >>= tz_b;
    let k = if tz_a < tz_b { tz_a } else { tz_b };
    while a != b {
        if a < b {
            (a, b) = (b, a);
        };
        a -= b;
        a >>= a.trailing_zeros();
    }
    a << k
}

fn solve(s: &str) -> usize {
    let b = s.bytes().map(|v| v - b'a').collect::<Vec<_>>();
    let freq_all = b.iter().fold([0; 26], |mut acc, &k| {
        acc[k as usize] += 1;
        acc
    });

    // somehow coderun defaults to 2018 edition
    let gcd_all = IntoIterator::into_iter(freq_all).filter(|&v| v != 0).fold(s.len(), gcd);
    let mut candidates = BTreeSet::new();
    let mut i = 1;
    while i * i <= gcd_all {
        let (d, rem) = (gcd_all / i, gcd_all % i);
        if rem == 0 {
            candidates.insert(Reverse(i));
            if d != i {
                candidates.insert(Reverse(d));
            }
        }
        i += 1;
    }
    candidates
        .into_iter()
        .find(|Reverse(d)| {
            let mut counts = b.chunks_exact(b.len() / d).map(|chunk| {
                chunk.iter().fold([0_u32; 26], |mut acc, &k| {
                    acc[k as usize] += 1;
                    acc
                })
            });
            let count = counts.next().unwrap();
            counts.all(|v| v == count)
        })
        .unwrap_or(Reverse(1))
        .0
}

fn main() {
    let s = std::io::stdin().lock().lines().next().unwrap().unwrap();
    println!("{}", solve(&s));
}
// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve("ababbaab"), 4);
    }
    #[test]
    fn test_1_x2() {
        assert_eq!(solve("ababbaabababbaab"), 8);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("ogorog"), 1);
    }
    #[test]
    fn test_2_x2() {
        assert_eq!(solve("ogorogogorog"), 2);
    }
}

// endregion: --- Tests
