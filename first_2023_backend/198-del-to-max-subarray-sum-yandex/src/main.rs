use std::{
    fmt::{Debug, Display},
    io::{self, BufRead, BufWriter, Error, Write},
    ops::{Add, AddAssign},
};

fn ka_c_rem_k<T>(a: &[T], k: usize) -> T
where
    T: Ord + Copy + Add<Output = T> + AddAssign + Default + Debug + Display,
{
    if a.is_empty() {
        return T::default();
    }
    if a.len() == 1 {
        return a[0];
    }

    let n = a.len();
    // upper index
    let mi = n - 1;

    // for standart kadane
    let mut ks = vec![vec![T::default(); k + 1]; 2];
    let mut ksm = vec![vec![T::default(); k + 1]; 2];

    // left sum removing up to k elements
    let mut ls = vec![vec![T::default(); k + 1]; n];
    // maximum subarray left sum after removing up to k elements
    let mut left_max = vec![vec![T::default(); k + 1]; n];

    // Compute maximum subarray right sum after removing up to k elements
    let mut rs = vec![vec![T::default(); k + 1]; n];
    let mut right_max = vec![vec![T::default(); k + 1]; n];

    for i in 0..n {
        for j in 0..=k {
            // for right sum
            let ri = mi - i;
            let prev_ri = ri + 1;
            if i == 0 {
                ks[i.c()][j] = a[i];
                ksm[i.c()][j] = a[i];

                ls[i][j] = a[i];
                left_max[i][j] = a[i].max(T::default());

                rs[ri][j] = a[ri];
                right_max[ri][j] = a[ri].max(T::default());

                rs[ri][j] = a[ri];

                if j != 0 {
                    ks[i.c()][j] = a[i];
                    ksm[i.c()][j] = a[i];

                    // i == 0
                    ls[i][j] = a[i].max(ls[i][j - 1]).max(T::default());
                    left_max[i][j] = a[i].max(T::default());

                    rs[ri][j] = a[ri].max(rs[ri][j - 1]).max(T::default());
                    right_max[ri][j] = a[ri].max(T::default());
                }
            } else if j == 0 {
                // текущий элемент или сумма пред+тек
                ks[i.c()][j] = a[i].max(ks[i.p()][j] + a[i]);
                ksm[i.c()][j] = ksm[i.p()][j].max(ks[i.c()][j]);

                // + left element
                ls[i][j] = ls[i - 1][j] + a[i];
                left_max[i][j] = ls[i][j].max(left_max[i - 1][j]);

                // + right element
                rs[ri][j] = rs[prev_ri][j] + a[ri];
                right_max[ri][j] = rs[ri][j].max(right_max[prev_ri][j]);
            } else {
                // макс без текущего элемента или с ним
                ks[i.c()][j] = a[i].max(ks[i.p()][j - 1]).max(ks[i.p()][j] + a[i]);
                ksm[i.c()][j] = ksm[i.p()][j].max(ks[i.c()][j]);

                // skip it or + left element
                ls[i][j] = ls[i - 1][j - 1].max(ls[i - 1][j] + a[i]);
                left_max[i][j] = left_max[i - 1][j].max(ls[i][j]);

                // skip it or + right element
                rs[ri][j] = rs[prev_ri][j - 1].max(rs[prev_ri][j] + a[ri]);
                right_max[ri][j] = right_max[prev_ri][j].max(rs[ri][j]);
            }
        }
    }

    let mut r = ksm[(n - 1).c()][k];
    if r < T::default() {
        return r;
    }
    drop(ksm);
    drop(ks);

    let mut la = vec![vec![T::default(); k + 1]; 2];
    let mut ra = vec![vec![T::default(); k + 1]; 2];

    la[0][0] = ls[0][0] + right_max[1][0];

    for i in 1..(n - 1) {
        for j in 0..=k {
            la[i.c()][j] = la[i.p()][j].max(ls[i][j] + right_max[i + 1][k - j]);
        }
    }

    for j in 0..=k {
        ra[(n - 1).c()][j] = rs[n - 1][j] + left_max[n - 2][k - j];
    }

    for i in (1..(n - 1)).rev() {
        ra[i.c()][0] = ra[i.n()][0].max(rs[i][0] + left_max[i - 1][0]);
    }

    for i in (1..(n - 1)).rev() {
        for j in 0..=k {
            ra[i.c()][j] = ra[i.n()][j].max(rs[i][j] + left_max[i - 1][k - j]);
        }
    }

    for j in 0..=k {
        r = r.max(la[(n - 2).c()][j]).max(ra[1.c()][j]);
    }
    r
}

fn run_me(a: &[isize], k: usize) -> isize {
    ka_c_rem_k(a, k)
}

fn main() -> Result<(), Error> {
    let mut out = BufWriter::new(std::io::stdout().lock());
    let mut line_iter = io::stdin().lock().lines().map_while(Result::ok);
    let num_tests = line_iter.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..num_tests {
        let k = line_iter.next().unwrap().split_whitespace().nth(1).unwrap().parse().unwrap();

        let ni =
            line_iter.next().unwrap().split_whitespace().flat_map(str::parse).collect::<Box<_>>();

        let res = run_me(&ni, k);
        writeln!(out, "{res:?}")?;
    }
    drop(line_iter);
    Ok(())
}

trait BinaryIndex {
    fn c(&self) -> usize;
    fn p(&self) -> usize;
    fn n(&self) -> usize;
}
impl BinaryIndex for usize {
    #[inline]
    fn c(&self) -> usize {
        self & 1
    }
    #[inline]
    fn p(&self) -> usize {
        (self - 1) & 1
    }
    #[inline]
    fn n(&self) -> usize {
        (self + 1) & 1
    }
}

#[cfg(test)]
mod test {
    use std::ops::Sub;

    use {super::*, rand::seq::SliceRandom, rand::Rng};

    fn kadane_linear_remove_k_sum<T>(a: &[T], k: usize) -> T
    where
        T: Ord + Debug + Display + Copy + Sub<Output = T> + Add<Output = T> + AddAssign + Default,
    {
        let mut max_el = a[0];
        let mut dp = vec![vec![T::default(); k + 1]; a.len()];

        dp[0][0] = T::default().max(a[0]);
        let mut res = dp[0][0];

        (1..a.len()).for_each(|i| {
            max_el = a[i].max(max_el);
            dp[i][0] = T::default().max(a[i] + dp[i - 1][0]);
            res = res.max(dp[i][0]);
            (1..=k).for_each(|j| {
                dp[i][j] = T::default().max(dp[i - 1][j - 1]).max(a[i] + dp[i - 1][j]);
                res = res.max(dp[i][j]);
            });
        });

        // #TODO: consider removing this
        (0..a.len()).for_each(|i| {
            (0..=k).for_each(|j| {
                res = res.max(dp[i][j]);
            });
        });

        // all array is negative
        if max_el < T::default() {
            max_el
        } else {
            res
        }
    }

    #[test]
    fn test_kadane_window() {
        let a = vec![-11, -2, -3, -1];

        assert_eq!(-1, ka_c_rem_k(&a, 1));
    }

    #[test]
    fn test_1() {
        let a = vec![1, -2, 9, 10];

        assert_eq!(20, run_me(&a, 1));
    }

    #[test]
    fn test_2() {
        let a = vec![5, -5, 5, -5, 5, -5];

        assert_eq!(10, run_me(&a, 1));
    }
    #[test]
    fn test_3() {
        let a = vec![5, -5, 5, -5, 5, -5];
        assert_eq!(15, run_me(&a, 2));
    }

    #[test]
    fn test_4() {
        let a = vec![5, -5, 5, -5];
        assert_eq!(10, run_me(&a, 1));
    }

    #[test]
    fn test_5() {
        let a = vec![-3, -1, 5, 6, -200, -200, 5, -1];
        assert_eq!(14, run_me(&a, 1));
    }

    #[test]
    fn test_6() {
        let a = vec![-1, -2, -3];
        assert_eq!(-1, run_me(&a, 1));
    }

    #[test]
    fn test_all_neg() {
        let a = vec![-1, -2, -3];
        assert_eq!(-1, run_me(&a, 1));
    }

    #[test]
    fn test_all_neg_k_around_len() {
        let a = vec![-1, -2, -3];
        assert_eq!(-1, run_me(&a, 2));
    }

    #[test]
    fn test_x() {
        let a = vec![1, -2, 3, -3];
        assert_eq!(4, run_me(&a, 2));
    }

    #[test]
    fn test_circ() {
        let a = vec![1, -2, -2, 3, -3, 1];
        assert_eq!(5, run_me(&a, 1));
    }

    #[test]
    fn test_all_positive() {
        let a = vec![1, 2, 3];
        assert_eq!(6, run_me(&a, 1));
    }

    #[test]
    fn test_totalsum_is_zero() {
        let a = vec![-1, 2, 1, -2, -3, 4, -4, 3];
        assert_eq!(10, run_me(&a, 4));
    }

    #[test]
    fn test_y() {
        let a = vec![-4, -3, -2, -1, 1, 0, -4, 3];
        assert_eq!(4, run_me(&a, 4));
    }

    #[test]
    fn test_wrapping() {
        let a = vec![1, -3, -200, 5, -2, 1, -1, 0, -4, 1];
        assert_eq!(8, run_me(&a, 3));
    }

    #[test]
    fn test_wrapping_a() {
        let mut a = vec![-2, 1, 3, -2, 4, -7, 20];
        a.reverse();
        assert_eq!(26, run_me(&a, 1));
    }
    #[test]
    fn test_fail1() {
        let a = vec![-15, 69, 37, 99, -10, -2, 2];
        assert_eq!(207, run_me(&a, 1));
    }

    #[test]
    fn test_fail2() {
        let a = vec![-56, -5, 48, -38, -20, 58, -94, 57, -55, 100];
        assert_eq!(258, run_me(&a, 3));
    }

    #[test]
    fn test_fail3() {
        let a = vec![-97, -39, 61, -85, -34, -66, -9, 50, -95];
        // <-- [-85, -34, -66, -9, 50, -95, -97, -39, 61] <-- 111
        // <-- [-34, -66, -9, 50, -95, -97, -39, 61, -85] <-- 111
        // <-- [-66, -9, 50, -95, -97, -39, 61, -85, -34] <-- 111
        // <-- [-9, 50, -95, -97, -39, 61, -85, -34, -66] <-- 111
        // <-- [50, -95, -97, -39, 61, -85, -34, -66, -9] <-- 111
        assert_eq!(111, run_me(&a, 3));
    }

    #[test]
    fn test_fail4() {
        let a = vec![81, -80, -28, 1, -32, -69, 28, -23, -23];
        // <-- [-80, -28, 1, -32, -69, 28, -23, -23, 81] <-- 109
        // <-- [-28, 1, -32, -69, 28, -23, -23, 81, -80] <-- 109
        // <-- [1, -32, -69, 28, -23, -23, 81, -80, -28] <-- 109
        // <-- [-32, -69, 28, -23, -23, 81, -80, -28, 1] <-- 109
        // <-- [-69, 28, -23, -23, 81, -80, -28, 1, -32] <-- 109
        // <-- [28, -23, -23, 81, -80, -28, 1, -32, -69] <-- 109
        assert_eq!(109, run_me(&a, 3));
    }

    #[test]
    fn test_fail5() {
        let a = vec![57, -88, 62, -63, 68, 10, -89, 98];
        // <-- [-88, 62, -63, 68, 10, -89, 98, 57] <-- 233
        // <-- [62, -63, 68, 10, -89, 98, 57, -88] <-- 233
        // <-- [-63, 68, 10, -89, 98, 57, -88, 62] <-- 233
        // <-- [68, 10, -89, 98, 57, -88, 62, -63] <-- 233
        assert_eq!(233, run_me(&a, 1));
    }
    #[test]
    fn test_fail6() {
        let a = vec![75, -90, -1, 72, -45, -40, 15, -21, 100];
        // <-- [-45, -40, 15, -21, 100, 75, -90, -1, 72] <-- 261
        // <-- [-40, 15, -21, 100, 75, -90, -1, 72, -45] <-- 261
        // <-- [15, -21, 100, 75, -90, -1, 72, -45, -40] <-- 261
        assert_eq!(261, run_me(&a, 2));
    }

    #[test]
    fn test_fail7() {
        let a = vec![50, -22, 6, -9, 38, -56];
        // <-- [-22, 6, -9, 38, -56, 50] <-- 88    51
        // <-- [6, -9, 38, -56, 50, -22] <-- 88    41
        // <-- [-9, 38, -56, 50, -22, 6] <-- 88    31
        // <-- [38, -56, 50, -22, 6, -9] <-- 88    21
        // <-- [-56, 50, -22, 6, -9, 38] <-- 85    51
        // <-- [50, -22, 6, -9, 38, -56] <-- 85    41
        assert_eq!(88, run_me(&a, 1));
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn huge_random_array_exact_k_negative() {
        let max_n = 7000_usize;
        let min_n = 1_usize;
        let min_k = 0_usize;
        let max_a: isize = 10_isize.pow(2);
        let min_a: isize = -max_a;

        let mut rng = rand::thread_rng();

        let mut a: Vec<isize> = Vec::with_capacity(max_n);
        let mut s = 0;

        // выбираем кол-во множителей
        // генерим рандомно K множителей пока M не попадает в нужные рамки
        // добавляем в вектор
        // генерим еще рандомных элементов и добавляем в массив
        // переставляем их местами
        let mut i = 0;
        loop {
            i += 1;
            let n = rng.gen_range(min_n..=max_n);
            let k = rng.gen_range(min_k..=100.min(n - 1));

            let mut total_sum: isize = 0;
            let mut pos_s = 0_isize;
            let mut neg_s = 0_isize;

            (0..k).for_each(|_i| {
                let ai = rng.gen_range(min_a..=-1);
                a.push(ai);
                neg_s += ai;
                total_sum += ai;
            });
            (k..n).for_each(|_i| {
                let ai = rng.gen_range(0..=max_a);
                a.push(ai);
                total_sum += ai;
                pos_s += ai;
            });

            a.shuffle(&mut rng);
            println!("{pos_s} + {neg_s} = {total_sum}, ");
            println!("testing k = {}", k / 2);
            //println!("{:?}", a);
            let res = run_me(&a, k / 2);
            //let mut np3 = Vec::from(np);
            let mut y = kadane_linear_remove_k_sum(&a, k / 2);
            for _i in 0..a.len() {
                a.rotate_left(1);
                //print!("<-- {:?} <--", a);
                let cr = kadane_linear_remove_k_sum(&a, k / 2);
                //println!(" {:<5}", cr);
                y = y.max(cr);
            }
            if y == res {
                s += 1;
            } else {
                //f += 1;
            };
            assert_eq!(y, res, "i = {} test {} got wrong with: k = {}", i, s + 1, k / 2);
            a.clear();
        }
    }
}
