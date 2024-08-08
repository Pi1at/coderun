use std::io::{self, BufRead};

fn run_me(input: &str, perm: usize) -> f64 {
    //prepare data
    let number = input.chars().rev().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
    #[allow(clippy::cast_precision_loss)]
    let n = number.len() as f64;
    let hand = number[0];
    let bi_2 = n * (n - 1.0) / 2.0;
    let (even_cnt, five_cnt, num_sum) =
        number.iter().fold((0_u32, 0_u32, 0_u32), |(e, c5, s), &x| match x {
            2 | 4 | 6 | 8 => (e + 1, c5, s + x),
            5 => (e, c5 + 1, s + x),
            _ => (e, c5, s + x),
        });

    let is_div_by_3 = 0 == num_sum % 3;
    // can divide by 5 or by 6
    let white = f64::from(five_cnt + if is_div_by_3 { even_cnt } else { 0 });

    let a_0 = 1.0 - 2.0 / (n - 1.0);
    let b_0 = white / bi_2;

    let p_0 = if (hand == 5) || (hand % 2 == 0 && is_div_by_3) { 1.0 } else { 0.0 };

    (1..=perm).fold(p_0, |acc, _| acc.mul_add(a_0, b_0))
}

fn main() {
    let mut line_iter = io::stdin().lock().lines();
    let input = line_iter.next().unwrap().unwrap();
    let k = line_iter.next().unwrap().unwrap().parse().unwrap();
    drop(line_iter);
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
