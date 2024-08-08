use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let result = io::stdin()
        .lock()
        .lines()
        .nth(1)
        .unwrap()
        .unwrap()
        .split_whitespace()
        .flat_map(str::parse)
        .fold(HashMap::<u32, u32>::new(), |mut m, k| {
            m.entry(k).and_modify(|count| *count += 1).or_insert(1);
            m
        })
        .into_values()
        .filter(|v| *v == 1)
        .count();
    println!("{result}");
}
