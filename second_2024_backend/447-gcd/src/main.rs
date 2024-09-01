use std::collections::HashMap;
use std::io::{self, BufRead};

// quick and dirty implementation
// TODO: dont't build new table, use callback or implement as trait
fn prime_factors(n: u64) -> HashMap<u64, u64> {
    let mut count = 0;
    let mut n = n;
    let mut factors = HashMap::new();
    // even
    // TODO: trailing zeroes?
    if n % 2 == 0 {
        while n % 2 == 0 {
            count += 1;
            n /= 2;
        }
        factors.insert(2, count);
    }
    // n odd
    let mut i = 3;
    count = 0;
    while i * i <= n {
        if n % i == 0 {
            while n % i == 0 {
                count += 1;
                n /= i;
            }
            factors.insert(i, count);
        }
        count = 0;
        i += 2;
    }
    if n != 1 {
        factors.insert(n, 1);
    }
    factors
}

fn solve(a: &[u32], b: &[u32], width: u32) -> String {
    // 10^9 for taking the last 9 digits
    let modulo: u64 = 10_u64.pow(width);

    let gcd_v = {
        let a_primes = a.iter().map(|&v| prime_factors(v.into())).fold(
            HashMap::<u64, u64>::new(),
            |mut acc, p_table| {
                for (k, v) in p_table {
                    acc.entry(k).and_modify(|c| *c += v).or_insert(v);
                }
                acc
            },
        );
        let b_primes = b.iter().map(|&v| prime_factors(v.into())).fold(
            HashMap::<u64, u64>::new(),
            |mut acc, p_table| {
                for (k, v) in p_table {
                    acc.entry(k).and_modify(|c| *c += v).or_insert(v);
                }
                acc
            },
        );
        // exp(k) = min(v_a,v_b)
        a_primes.into_iter().fold(Vec::<(u64, u64)>::new(), |mut acc, (k, a_count)| {
            if let Some(b_count) = b_primes.get(&k) {
                acc.push((k, a_count.min(*b_count)));
            }
            acc
        })
    };

    let (result, overflow) =
        gcd_v.into_iter().fold((1_u64, false), |(mut acc, mut overflow), (v, count)| {
            for _ in 0..count {
                acc *= v;
                // 1_000_000_000
                // 999_999_999 - maximum valid value
                overflow = (acc >= modulo) || overflow;
                if overflow {
                    acc %= modulo;
                }
            }
            (acc, overflow)
        });
    if overflow {
        format!("{result:0width$}", width = width as usize)
    } else {
        format!("{result}")
    }
}

fn main() {
    let mut input = io::stdin().lock().lines().map_while(Result::ok);
    let a = input.nth(1).unwrap().split_ascii_whitespace().flat_map(str::parse).collect::<Vec<_>>();
    let b = input.nth(1).unwrap().split_ascii_whitespace().flat_map(str::parse).collect::<Vec<_>>();
    let ans = solve(&a, &b, 9);
    drop(input);
    println!("{ans}");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(&[2, 3, 5], &[4, 5], 9), "10");
    }
}
