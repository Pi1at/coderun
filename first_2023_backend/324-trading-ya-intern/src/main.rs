use std::{
    cmp::Reverse,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines().map_while(Result::ok);
    let n_or_m = line_iter.next().unwrap().split_whitespace().flat_map(str::parse).min().unwrap();

    let mut sellers =
        line_iter.next().unwrap().split_whitespace().flat_map(str::parse).collect::<Vec<usize>>();
    sellers.sort_unstable();

    let mut buyers =
        line_iter.next().unwrap().split_whitespace().flat_map(str::parse).collect::<Vec<usize>>();
    drop(line_iter);
    buyers.sort_unstable_by_key(|&k| Reverse(k));

    let res: usize =
        sellers.into_iter().zip(buyers).take(n_or_m).map_while(|(s, b)| b.checked_sub(s)).sum();
    println!("{res}");
}
