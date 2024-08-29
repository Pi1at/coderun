use std::{
    cmp,
    io::{self, BufRead},
};

fn find_div_nn(numbers: &[(usize, usize)], i: usize, k: usize, m: usize) -> Option<Vec<usize>> {
    let n = numbers.len();
    let last_i = n - k;

    if i >= n {
        return None;
    }

    if k == 0 {
        return (m == 1).then_some(Vec::<usize>::new());
    }

    let mut cur_i = i;
    while (numbers[cur_i].1 > m) && (cur_i <= last_i) {
        cur_i += 1;
    }

    // at this point numbers[cur_i].1 <= m;
    // if we need find k one's in slice
    if m == 1 {
        return (cur_i <= last_i).then_some((cur_i..cur_i + k).map(|idx| numbers[idx].0).collect());
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
        let mut b = a.iter().filter(|&&(i, num)| (i <= k) || num == 0).collect::<Vec<_>>();
        b.sort_unstable_by_key(|&&(_, num)| num);
        return b.iter().take(k).map(|&&(idx, _)| idx + 1).collect::<Vec<_>>();
    }

    // m > 0, lets filter out some obv and sort in descend order

    let mut b = a
        .iter()
        .filter_map(|&(idx, num)| (num <= m).then_some((idx, num)))
        .collect::<Vec<(_, _)>>();

    b.sort_unstable_by_key(|&(_, num)| cmp::Reverse(num));

    let ans = find_div_nn(&b, 0, k, m);

    ans.map_or_else(Vec::new, |x| x.iter().map(|&idx| (idx + 1)).collect::<Vec<_>>())
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
        .flat_map(str::parse)
        .collect::<Vec<_>>();
    let a = line_iter
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .flat_map(str::parse)
        .enumerate()
        .collect::<Vec<_>>();
    drop(line_iter);
    let res = (find_k_eq_m(&a, mk[1], mk[0]))
        .iter()
        .map(|&idx| idx.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{res}");
}

#[cfg(test)]
mod test {

    use std::time;

    use super::*;
    use rand::{seq::SliceRandom, Rng};

    #[test]
    #[allow(clippy::many_single_char_names)]
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
                }
                a.clear();
            }
            println!("--------------------");
            //print!("множители {:?}", a);
            for _x in 0..(n - a.len()) {
                a.push(rng.gen_range(min_m..=300));
            }
            a.shuffle(&mut rng);

            println!();
            let b = a.iter().copied().enumerate().collect::<Vec<_>>();

            println!("N = {n} K = {k}");
            print!("FIND {m} =");
            let before = time::Instant::now();
            let ans = find_k_eq_m(&b, k, m);
            let d = before.elapsed();
            println!("Elapsed time: {:.2?}", before.elapsed());
            assert!(d.as_secs() < 1, "TL");

            let res =
                if ans.is_empty() { 0 } else { ans.iter().fold(1, |acc, idx| acc * a[idx - 1]) };
            println!("res {res:?}");
            assert_eq!(res, m);
        }
    }
}
