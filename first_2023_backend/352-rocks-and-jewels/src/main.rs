use std::{
    collections::HashSet,
    io::{self, BufRead},
    iter::FromIterator,
};
fn main() {
    let stdin = io::stdin();
    let line_iter = stdin.lock().lines();
    let kv = line_iter.take(2).flatten().collect::<Vec<_>>();

    let u = HashSet::<char>::from_iter(kv[0].chars());

    let result = u.iter().fold(0, |total, &c1| {
        total
            + kv[1]
                .chars()
                .fold(0, |b, c2| b + if c1 == c2 { 1 } else { 0 })
    });
    println!("{}", result);
}
