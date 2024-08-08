use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    // В первой строке заданы три целых числа
    let mut r: Vec<isize> = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .flat_map(str::parse)
        .collect();
    // median fn you should use, but it is what it is
    r.sort_unstable();
    println!("{}", r[1]);
}
