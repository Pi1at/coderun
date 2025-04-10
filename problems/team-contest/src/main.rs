use std::io::BufRead;

use lib::PartialSum;

fn solve_step(
    costs: &[Vec<u32>],
    p_sums: &[PartialSum<u32>],
    [first, mid, last]: [usize; 3],
) -> u32 {
    let n = costs[0].len();

    let mid_sum = &p_sums[mid];
    let first_sum = &p_sums[first];
    let mut min_cost = u32::MAX;

    let mut dp = vec![0; n];
    // start from mid pivot into last
    dp[1] = costs[mid][0] + costs[last][1];
    for idx in 2..n {
        // continue previous path or restart (pivot on last here)
        dp[idx] = dp[idx - 1].min(mid_sum.sum(..idx)) + costs[last][idx];
        // pivot on first
        min_cost = min_cost.min(first_sum.sum(idx..) + dp[idx - 1]);
    }
    min_cost
}

fn solve(costs: &[Vec<u32>]) -> u32 {
    let permutations: [[usize; 3]; 6] =
        [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
    let p = costs.iter().map(|v| PartialSum::new(v)).collect::<Vec<_>>();
    permutations.into_iter().map(|order| solve_step(costs, &p, order)).min().unwrap()
}

fn main() {
    let costs: Vec<Vec<_>> = std::io::stdin()
        .lock()
        .lines()
        .skip(1)
        .map_while(Result::ok)
        .take(3)
        .map(|line| line.split_ascii_whitespace().flat_map(str::parse).collect::<Vec<_>>())
        .collect();
    assert_eq!(3, costs.len(), "input is malformed");
    let ans = solve(&costs);
    println!("{ans}");
}

// region:    --- Tests

#[cfg(test)]
mod tests {

    use lib::join_into_string;
    use rand::distr::Uniform;
    use rand::{Rng, rng};

    use super::*;

    fn parse_and_solve(s: &[&str]) -> u32 {
        let costs: Vec<Vec<_>> = s
            .iter()
            .take(3)
            .map(|line| {
                line.split_ascii_whitespace().flat_map(str::parse::<u32>).collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(3, costs.len(), "input is malformed");
        solve(&costs)
    }

    fn solve_naive(a: &[Vec<u32>]) -> u32 {
        let p_a = PartialSum::new(&a[0]);
        let p_b = PartialSum::new(&a[1]);
        let p_c = PartialSum::new(&a[2]);
        assert!(p_a.len() == p_b.len() && p_b.len() == p_c.len());
        let n = p_a.len();
        let mut min_difficulty = u32::MAX;

        // n is len!, so last_idx = n-1
        // i - start index of second interval
        // j - start index of third interval
        for i in 1..n - 1 {
            let a_1 = p_a.sum(..i);
            let b_1 = p_b.sum(..i);
            let c_1 = p_c.sum(..i);
            for j in (i + 1)..n {
                // Calculate total difficulty for this (i, j)
                let a_2 = p_a.sum(i..j);
                let a_3 = p_a.sum(j..);
                let b_2 = p_b.sum(i..j);
                let b_3 = p_b.sum(j..);
                let c_2 = p_c.sum(i..j);
                let c_3 = p_c.sum(j..);
                let v1 = a_1 + b_2 + c_3;
                let v2 = a_1 + c_2 + b_3;
                let v3 = b_1 + a_2 + c_3;
                let v4 = b_1 + c_2 + a_3;
                let v5 = c_1 + a_2 + b_3;
                let v6 = c_1 + b_2 + a_3;
                min_difficulty = min_difficulty.min(v1).min(v2).min(v3).min(v4).min(v5).min(v6);
            }
        }

        min_difficulty
    }

    #[test]
    fn test_1() {
        assert_eq!(parse_and_solve(&["1 3 3", "1 1 1", "1 2 3"]), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(parse_and_solve(&["3 3 4 1 3 4 4", "4 2 5 1 5 5 4", "5 5 1 3 4 4 4"]), 19);
    }

    #[test]
    fn find_wa() {
        let arr_len = 150;
        let mut costs = vec![Vec::new(); 3];
        let mut rng = rng();
        let diff_range = Uniform::new_inclusive(1_u32, 5_u32).unwrap();
        loop {
            (0..3).for_each(|idx| {
                costs[idx] = (&mut rng).sample_iter(diff_range).take(arr_len).collect::<Vec<_>>();
            });

            let res1 = solve_naive(&costs);
            let res2 = solve(&costs);
            if res1 != res2 {
                println!("answer {res1} but got {res2}");
                println!("a {}", join_into_string(&costs[0]));
                println!("b {}", join_into_string(&costs[1]));
                println!("c {}", join_into_string(&costs[2]));
                break;
            }
        }
    }
}

// endregion: --- Tests

mod lib {
    #![allow(dead_code)]
    pub fn join_into_string<T>(a: &[T]) -> String
    where T: ToString {
        let Some((first, suffix)) = a.split_first() else { return String::new() };
        let first_owned = first.to_string();
        suffix.iter().fold(first_owned, |mut a, b| {
            a.push(' ');
            a.push_str(&b.to_string());
            a
        })
    }

    use std::ops::RangeBounds;

    #[derive(Debug)]
    pub struct PartialSum<T> {
        sum: Vec<T>,
    }
    impl<T> PartialSum<T>
    where T: Copy + Default + std::ops::Add<Output = T> + std::ops::Sub<Output = T>
    {
        pub fn new(v: &[T]) -> Self {
            let mut sum = vec![T::default(); v.len() + 1];

            for i in 0..v.len() {
                sum[i + 1] = sum[i] + v[i];
            }
            Self { sum }
        }

        pub fn len(&self) -> usize { self.sum.len() - 1 }

        /// Sum of elements
        /// sum(..k) sum of first k-1 elements
        /// sum(..=k) sum of first k elements
        /// sum(k..) sum of elements starting
        pub fn sum<Idx>(&self, index: Idx) -> T
        where Idx: RangeBounds<usize> {
            let start = match index.start_bound() {
                std::ops::Bound::Included(i) => *i,
                std::ops::Bound::Excluded(i) => *i + 1,
                std::ops::Bound::Unbounded => 0,
            };
            let end = match index.end_bound() {
                std::ops::Bound::Included(j) => *j + 1,
                std::ops::Bound::Excluded(j) => *j,
                std::ops::Bound::Unbounded => self.len(),
            };
            self.sum[end] - self.sum[start]
        }

        pub fn inner(&self) -> &[T] { &self.sum }
    }

    impl<T> FromIterator<T> for PartialSum<T>
    where T: Copy + Default + std::ops::Add<Output = T> + std::ops::Sub<Output = T>
    {
        fn from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> Self {
            let iter = into_iter.into_iter();
            let mut sum: Vec<T> = if let (_, Some(n)) = iter.size_hint() {
                Vec::with_capacity(n + 1)
            } else {
                Vec::new()
            };
            sum.push(T::default());
            for v in iter {
                let p = sum.last().map_or(v, |last| *last + v);
                sum.push(p);
            }
            Self { sum }
        }
    }

    #[test]
    fn compare_new_vs_iter() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let p1 = PartialSum::new(&v);
        for i in 0..v.len() {
            println!("i: {i} sum: {}", p1.sum(i..=i));
        }
        let p2 = v.into_iter().collect::<PartialSum<_>>();
        assert!(p1.sum.into_iter().zip(p2.sum.into_iter()).all(|(v1, v2)| v1 == v2));
    }
}
