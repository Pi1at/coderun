use std::io::{self, BufRead};

fn total_prime_factors(n: u64) -> u64 {
    let mut count = 0;
    let mut n = n;
    // четное
    if n % 2 == 0 {
        count += 1;
        while n % 2 == 0 {
            n /= 2;
        }
    }

    // n нечетное
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            count += 1;
            while n % i == 0 {
                n /= i;
            }
        }
        i += 2;
    }

    if n > 2 {
        count += 1;
    }
    count
}

fn count_pairs(g: u64, l: u64) -> u64 {
    if l % g != 0 {
        0 // без остатка не делится
    } else {
        1 << total_prime_factors(l / g)
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    let numbers: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let (g, l) = (numbers[0], numbers[1]);

    let result = count_pairs(g, l);

    println!("{}", result);
}
