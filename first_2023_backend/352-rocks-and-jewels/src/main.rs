use std::{
    collections::HashSet,
    io::{self, BufRead},
};
fn main() {
    // TODO: just collect hashset and fold second line
    let kv = io::stdin().lock().lines().map_while(Result::ok).take(2).collect::<Vec<_>>();
    let u = kv[0].chars().collect::<HashSet<char>>();
    let result = u
        .into_iter()
        .fold(0, |total, c1| total + kv[1].chars().fold(0, |b, c2| b + i32::from(c1 == c2)));
    println!("{result}");
}
