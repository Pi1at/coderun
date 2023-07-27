// 313. Сумма медиан
use std::{
    io::{self, BufRead},
    println,
};

use std::collections::BinaryHeap;

fn run_me(arr: &[isize]) -> isize {
    // bh returns max element, and we need minimum of maximums so lets play
    let mut g = BinaryHeap::<_>::new();
    let mut s = BinaryHeap::<_>::new();
    let mut result = 0;
    for &el in arr {
        s.push(el);
        g.push(-s.pop().unwrap());
        if g.len() > s.len() {
            s.push(-g.pop().unwrap());
        }
        result += s.peek().unwrap();
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();
    let _n = line_iter.next().unwrap().unwrap().parse::<usize>().unwrap();

    let xi = line_iter
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .flat_map(|s| s.parse::<isize>())
        .collect::<Vec<_>>();
    println!("{}", run_me(&xi));
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use {super::*, core::panic, rand::seq::SliceRandom, rand::Rng};
}
