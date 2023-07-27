use std::{
    io::{self, BufRead},
    iter::FromIterator,
};

fn main() {
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();
    // В первой строке заданы три целых числа
    let mut r = Vec::from_iter(
        line_iter
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<isize>().unwrap()),
    );
    r.sort();
    println!("{}", r[1]);
}