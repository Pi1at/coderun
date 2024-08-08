use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn horners_method(coefficients: &[i32], x: i32) -> i32 {
    let degree = coefficients.len() - 1;
    let mut result = coefficients[degree];
    for i in (0..degree).rev() {
        result = result * x + coefficients[i];
    }
    result
}

fn modify_number(sign: i32, number: &mut Vec<i32>, powers: &mut HashMap<u32, i32>) {
    for (pos, e) in number.drain(..).enumerate() {
        *powers.entry(pos as u32).or_default() += e * sign;
    }
}
// Who needs lexers
fn run_me(input: &str) -> i32 {
    let mut min_base: i32 = 2;
    let mut sign = 1;
    let mut powers: HashMap<u32, i32> = HashMap::new();
    let mut number = Vec::new();

    for index in input.chars().rev() {
        match index {
            'A'..='Z' | '0'..='9' => {
                let i: i32 = index.to_digit(36).unwrap() as i32;
                min_base = min_base.max(i + 1);
                number.push(i);
            }
            '-' => {
                modify_number(-sign, &mut number, &mut powers);
            }
            '=' => {
                modify_number(sign, &mut number, &mut powers);
                sign = -1;
            }
            '+' => {
                modify_number(sign, &mut number, &mut powers);
            }
            _ => {}
        }
    }
    modify_number(sign, &mut number, &mut powers);

    let mut prepare = powers.into_iter().collect::<Vec<_>>();
    prepare.sort_by_key(|el| el.0);
    let coeff =
        prepare.into_iter().skip_while(|(_, v)| *v == 0).map(|(_, v)| v).collect::<Vec<_>>();

    match coeff.first() {
        Some(&x) => (min_base..=x.abs())
            .find(|b| (x % b == 0) && (horners_method(&coeff, *b) == 0))
            .unwrap_or(-1),
        None => min_base,
    }
}

fn main() {
    let input = io::stdin().lock().lines().next().unwrap().unwrap();
    let result = run_me(&input);
    println!("{result}");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn base_11_long() {
        assert_eq!(11, run_me("AAAAAAAAA9A+10=AAAAAAAAAAA"));
    }
    #[test]
    fn base_3() {
        assert_eq!(3, run_me("10 + 10 + 10 = 100"));
    }
    #[test]
    fn base_3r() {
        assert_eq!(3, run_me("100 = 10 + 10 + 10"));
    }

    #[test]
    fn base_8() {
        assert_eq!(8, run_me("106 + 74 = 202"));
    }
    #[test]
    fn base_x() {
        assert_eq!(8, run_me("137 + 144 = 303"));
    }

    #[test]
    fn base_4() {
        assert_eq!(4, run_me("2 + 2 = 11 - 1"));
    }
    #[test]
    fn base_5() {
        assert_eq!(5, run_me("2 + 3 = 11 - 1"));
    }
    #[test]
    fn base_2() {
        assert_eq!(2, run_me("1 = 1"));
    }
    #[test]
    fn base_none() {
        assert_eq!(-1, run_me("2 = 3"));
    }
    #[test]
    fn base_12() {
        assert_eq!(12, run_me("B = A + 1"));
    }
    #[test]
    fn base_16() {
        assert_eq!(16, run_me("F - 1 = E"));
    }
    #[test]
    fn base_4r() {
        assert_eq!(4, run_me("11 - 1 = 2 + 2"));
    }

    #[test]
    fn base_12r() {
        assert_eq!(12, run_me("A + 1 = B"));
    }
    #[test]
    fn base_16r() {
        assert_eq!(16, run_me("E = F - 1"));
    }

    #[test]
    fn horn() {
        let coefficients = vec![2, -1, 3, 0];
        let x = 2; //
        let result = horners_method(&coefficients, x);
        assert_eq!(12, result);
    }
}
