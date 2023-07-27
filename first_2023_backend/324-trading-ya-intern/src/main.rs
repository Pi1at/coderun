use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();
    let nm = line_iter
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .flat_map(|s| s.parse::<usize>())
        .collect::<Vec<_>>();

    let mut sellers = line_iter
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    sellers.sort_unstable();

    let mut buyers = line_iter
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    buyers.sort_unstable_by_key(|k| -k);

    let res: isize = sellers
        .iter()
        .zip(buyers.iter())
        .take(nm[0].min(nm[1]))
        .map_while(|(s, b)| match b - s {
            x if x > 0 => Some(x),
            _ => None,
        })
        .sum();
    println!("{}", res);
}
