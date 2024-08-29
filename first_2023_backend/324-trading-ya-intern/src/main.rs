use core::cmp::Reverse;
use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines().map_while(Result::ok);
    let n_or_m = lines.next().unwrap().split_whitespace().flat_map(str::parse).min().unwrap();

    let mut sellers =
        lines.next().unwrap().split_whitespace().flat_map(str::parse).collect::<Vec<usize>>();
    sellers.sort_unstable();

    let mut buyers =
        lines.next().unwrap().split_whitespace().flat_map(str::parse).collect::<Vec<usize>>();
    drop(lines);
    buyers.sort_unstable_by_key(|&k| Reverse(k));

    let res: usize =
        sellers.into_iter().zip(buyers).take(n_or_m).map_while(|(s, b)| b.checked_sub(s)).sum();
    println!("{res}");
}
