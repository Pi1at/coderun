use std::io::{self, BufRead};

const fn phi(mut n: u32) -> u32 {
    let mut result = n;
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            while n % i == 0 {
                n /= i;
            }
            result -= result / i;
        }
        i += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    result
}

fn main() {
    let n = io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .expect("Please enter a valid number");
    let result = phi(n);
    println!("{result}");
}
