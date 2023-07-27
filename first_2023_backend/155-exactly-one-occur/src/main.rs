use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut m: HashMap<u32, u32> = HashMap::new();

    stdin
        .lock()
        .lines()
        .nth(1)
        .unwrap()
        .unwrap()
        .split_whitespace()
        .for_each(|s| {
            let k = s.parse().unwrap();
            m.insert(k, 1 + if m.contains_key(&k) { m[&k] } else { 0 });
        });

    let result = m.values().filter(|v| **v == 1).count();
    println!("{}", result);
}
