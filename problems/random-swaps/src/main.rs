use std::io::{self, BufRead};

fn run_me(input: &str, perm: usize) -> f64 {
    //prepare data
    let number = input.chars().rev().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>();
    let n = u32::try_from(number.len()).expect("string too long");
    let hand = number[0];
    let bi_2 = n * (n - 1) / 2; // n*(n-1) is even
    let (even_cnt, five_cnt, num_sum) =
        number.iter().fold((0_u32, 0_u32, 0_u32), |(e, c5, s), &x| match x {
            2 | 4 | 6 | 8 => (e + 1, c5, s + x),
            5 => (e, c5 + 1, s + x),
            _ => (e, c5, s + x),
        });

    let is_div_by_3 = 0 == num_sum % 3;
    // can divide by 5 or by 6
    let white = f64::from(five_cnt + if is_div_by_3 { even_cnt } else { 0 });

    let a_0 = 1.0 - 2.0 / f64::from(n - 1);
    let b_0 = white / f64::from(bi_2);
    let p_0 = if (hand == 5) || (hand % 2 == 0 && is_div_by_3) { 1.0 } else { 0.0 };

    (1..=perm).fold(p_0, |acc, _| acc.mul_add(a_0, b_0))
}

fn main() {
    let mut lines = io::stdin().lock().lines().map_while(Result::ok);
    let input = lines.next().unwrap();
    let k = lines.next().unwrap().parse().unwrap();
    drop(lines);
    let result = run_me(&input, k);
    println!("{result}");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        assert!((1.0_f64 - run_me("21", 1)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_2() {
        assert!((1.0_f64 / 3.0 - run_me("145", 2)).abs() < f64::EPSILON);
    }
}
