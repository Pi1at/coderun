// some theory
// https://math.stackexchange.com/questions/222674/average-bus-waiting-time
use std::io::{self, BufRead};
use std::ops::{AddAssign, Mul};
use std::println;

fn multiply_polynomials<T>(a: &[T], b: &[T]) -> Vec<T>
where T: Mul<Output = T> + Default + Copy + AddAssign {
    let mut c = vec![T::default(); a.len() + b.len() - 1];
    (0..a.len()).for_each(|i| {
        (0..b.len()).for_each(|j| {
            c[i + j] += a[i] * b[j];
        });
    });
    c
}

// (t1-x)*(t2-x)*,,,*(tk-x)
fn multiply_t(t: &[isize]) -> Vec<isize> {
    // each t[i] - it's like [t[i];-1]
    let mut res = vec![0; 2];
    res[0] = t[0];
    res[1] = -1;

    for &c in &t[1..] {
        res = multiply_polynomials(&res, &[c, -1]);
    }
    res
}

fn power_up(p: &[isize]) -> (Vec<isize>, isize) {
    #![allow(clippy::cast_possible_wrap)]
    // for each p[i]*x^i
    // z[i+1] = p[i]*(p/i+1)
    // q[i+1] =
    // calculate common div
    let mut cd = 1;
    let mut max_pow = 0;
    let mut non_zero_pows = Vec::new();
    (0..p.len()).for_each(|i| {
        if p[i] != 0 {
            max_pow = i;
            cd *= (i + 1) as isize;
            non_zero_pows.push(i);
        }
    });
    let mut res = vec![0; max_pow + 2];
    for cp in non_zero_pows {
        res[cp + 1] = p[cp] * cd / (cp as isize + 1);
    }
    (res, cd)
}

fn horners_method(coefficients: &[isize], x: isize) -> isize {
    let max_pow = coefficients.len() - 1;
    let mut result = coefficients[max_pow];
    for i in (0..max_pow).rev() {
        result = result * x + coefficients[i];
    }
    result
}

const fn gcd_euclid(x: isize, y: isize) -> isize {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn run_me(a: &[isize]) -> String {
    // find minimum value - this will be x
    // "multiply" all tks
    // "take integral" over parts of polynome
    // calculate via gorner with value x
    // find gcd
    // PROFIT

    let &min_t = a.iter().min().unwrap();
    let p: isize = a.iter().product();

    let prob_polynom = multiply_t(a);
    let (e_poly, div_1) = power_up(&prob_polynom);
    let nominator = horners_method(&e_poly, min_t);
    let denominator = p * div_1;
    let g = gcd_euclid(nominator, denominator);
    format!("{}/{}", nominator / g, denominator / g)
}

/*
Первая строка входных данных содержит единственное число N — количество маршрутов (1≤N≤5).
Вторая строка содержит N целых положительных чисел tk​ (1≤tk≤50).
 */
fn main() {
    let tk = io::stdin()
        .lock()
        .lines()
        .nth(1)
        .unwrap()
        .unwrap()
        .split_whitespace()
        .flat_map(str::parse)
        .collect::<Vec<_>>();
    println!("{}", run_me(&tk));
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use {super::*, core::panic};

    #[test]
    fn test_50_49_48_47_46() {
        // 1/(t1*t2*...)
        let a = vec![50, 49, 48, 47, 46];
        let &min_t = a.iter().min().unwrap();
        let p: isize = a.iter().product();

        let prob_polynom = multiply_t(&a);
        let (e_poly, div_1) = power_up(&prob_polynom);
        let up = horners_method(&e_poly, min_t);
        let down = p * div_1;
        let g = gcd_euclid(up, down);
        println!("{up}/{down} g = {g}");
        assert_eq!("2650175/331632", format!("{}/{}", up / g, down / g));
    }
}
