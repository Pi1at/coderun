use std::io::BufRead;
// TODO: reimplement all
fn solve_naive(n: u32) -> (u32, u32) {
    fn count_divisors(x: u32) -> u32 {
        let mut count = 0;
        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        let sqrt_x = f64::from(x).sqrt() as u32;

        for i in 1..=sqrt_x {
            if x % i == 0 {
                count += 1; // i is a divisor
                if i != x / i {
                    count += 1; // x / i is also a divisor
                }
            }
        }

        count
    }

    let mut max_d = 0;
    let mut num = 0;

    for i in 1..=n {
        let divisors_count = count_divisors(i);
        if divisors_count > max_d || (divisors_count == max_d && i > num) {
            max_d = divisors_count;
            num = i;
        }
    }

    (num, max_d)
}

fn main() {
    let n: u32 = std::io::stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();
    let (number, count) = solve_naive(n);
    println!("{number}\n{count}");
}
use std::io::BufRead;
// TODO: reimplement all
fn solve_naive(n: u32) -> (u32, u32) {
    fn count_divisors(x: u32) -> u32 {
        let mut count = 0;
        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        let sqrt_x = f64::from(x).sqrt() as u32;

        for i in 1..=sqrt_x {
            if x % i == 0 {
                count += 1; // i is a divisor
                if i != x / i {
                    count += 1; // x / i is also a divisor
                }
            }
        }

        count
    }

    let mut max_d = 0;
    let mut num = 0;

    for i in 1..=n {
        let divisors_count = count_divisors(i);
        if divisors_count > max_d || (divisors_count == max_d && i > num) {
            max_d = divisors_count;
            num = i;
        }
    }

    (num, max_d)
}

fn main() {
    let n: u32 = std::io::stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();
    let (number, count) = solve_naive(n);
    println!("{number}\n{count}");
}
