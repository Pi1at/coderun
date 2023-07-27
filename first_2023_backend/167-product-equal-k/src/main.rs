use std::{
    io::{self, BufRead},
    iter::FromIterator,
    println,
};

fn find_div_nn(numbers: &[(usize, usize)], i: usize, k: usize, m: usize) -> Option<Vec<usize>> {
    let n = numbers.len();
    let last_i = n - k;

    if i >= n {
        return None;
    }

    if k == 0 {
        if m == 1 {
            return Some(Vec::<usize>::new());
        }
        return None;
    }

    let mut cur_i = i;

    while (numbers[cur_i].1 > m) && (cur_i <= last_i) {
        cur_i += 1;
    }

    // at this point numbers[cur_i].1 <= m;
    // if we need find k one's in slice
    if m == 1 {
        if cur_i <= last_i {
            return Some((cur_i..cur_i + k).map(|idx| numbers[idx].0).collect());
        } else {
            return None;
        }
    }
    if numbers[cur_i].1 == 1 {
        return None;
    }

    // found divisor of m
    if m % numbers[cur_i].1 == 0 {
        // try to find next num
        if let Some(mut x) = find_div_nn(numbers, cur_i + 1, k - 1, m / numbers[cur_i].1) {
            x.push(numbers[cur_i].0);
            return Some(x);
        }
    };

    find_div_nn(numbers, cur_i + 1, k, m)
}

fn find_k_eq_m(a: &[(usize, usize)], k: usize, m: usize) -> Vec<usize> {
    // given answer is exist

    // m = 0, peek k-1 numbers and 0
    if m == 0 {
        let mut b = a
            .iter()
            .filter(|(i, num)| (*i <= k) || *num == 0)
            .collect::<Vec<_>>();
        b.sort_unstable_by_key(|(_, num)| *num);
        b.truncate(k);
        return b.iter().map(|(idx, _)| idx + 1).collect::<Vec<_>>();
    }

    // m > 0, lets filter out some obv and sort in descend order

    let mut b = Vec::from_iter(a.iter().filter_map(
        |&(idx, num)| {
            if num <= m {
                Some((idx, num))
            } else {
                None
            }
        },
    ));

    b.sort_unstable_by_key(|&(_, num)| std::cmp::Reverse(num));

    let ans = find_div_nn(&b, 0, k, m);

    ans.map_or_else(Vec::new, |x| {
        x.iter().map(|&idx| (idx + 1)).collect::<Vec<_>>()
    })
}

fn main() {
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();
    let mk = line_iter
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .flat_map(|s| s.parse::<usize>())
        .collect::<Vec<_>>();
    let a = line_iter
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .flat_map(|s| s.parse::<usize>())
        .enumerate()
        .collect::<Vec<_>>();

    let res = (find_k_eq_m(&a, mk[1], mk[0]))
        .iter()
        .map(|&idx| (idx).to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", res);
}

#[cfg(test)]
mod test {
    use core::panic;

    use super::*;
    use rand::{seq::SliceRandom, Rng};

    
    #[test]
    fn random_array() {
        let max_m = 10_usize.pow(9);
        let min_m = 1_usize;
        let max_nk = 5000_usize;
        let min_nk = 1_usize;

        let mut rng = rand::thread_rng();
        let mut m: usize;
        let mut a: Vec<usize> = Vec::with_capacity(max_nk);

        // выбираем кол-во множителей
        // генерим рандомно K множителей пока M не попадает в нужные рамки
        // добавляем в вектор
        // генерим еще рандомных элементов и добавляем в массив
        // переставляем их местами
        loop {
            let k = rng.gen_range(min_nk..=max_nk);
            let n = rng.gen_range(k..=max_nk);

            loop {
                let mut new_max = max_m;
                for _z in 0..k {
                    let ai = rng.gen_range(min_m..=new_max);
                    a.push(ai);
                    if ai != 0 {
                        new_max /= ai;
                    } else {
                        new_max /= 2;
                    };
                }
                m = a.iter().fold(1, |acc, &x| acc.saturating_mul(x));
                if (m <= max_m) && (m >= min_m) {
                    break;
                } else {
                    a.drain(..);
                }
            }
            println!("--------------------");
            //print!("множители {:?}", a);
            for _x in 0..(n - a.len()) {
                a.push(rng.gen_range(min_m..=300))
            }
            a.shuffle(&mut rng);

            //println!("{:?}", factorize_all(&a));

            //prime_factors(m);
            println!();
            let b = a
                .iter()
                .enumerate()
                .map(|(k, &v)| (k, v))
                .collect::<Vec<_>>();

            //println!("{:?}", a);
            //println!("{:?}", b);
            println!("N = {n} K = {k}");
            print!("FIND {m} =");
            let before = std::time::Instant::now();
            let ans = find_k_eq_m(&b, k, m);
            let d = before.elapsed();
            println!("Elapsed time: {:.2?}", before.elapsed());
            if d.as_secs() >= 1 {
                panic!()
            };
            //println!("ANSWER {:?}", ans);
            let res = if !ans.is_empty() {
                ans.iter().fold(1, |acc, idx| acc * a[idx - 1])
            } else {
                //println! {"NOT FOUND!"};
                // let c = Vec::from_iter(b.iter().map(|(k, v)| *v));
                // println!("{:?}", c);
                0
            };
            println!("res {:?}", res);
            // let res = ans.map_or_else(
            //     || 0,
            //     |v| {
            //         v.iter()
            //             .map(|i| {
            //                 print!("{:} ", a[*i]);
            //                 a[*i] as u128
            //             })
            //             .product()
            //     },
            // );

            // if res != m {
            //     break;
            // };
            assert_eq!(res, m);
        }
    }
}
