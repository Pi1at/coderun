use std::{
    io::{self, BufRead},
    println,
};

fn main() {
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();
    let serv_count: usize = line_iter.next().unwrap().unwrap().parse().unwrap();
    let mut server_prob: Vec<f64> = vec![0_f64; serv_count];

    for serv in server_prob.iter_mut().take(serv_count) {
        *serv = line_iter
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<f64>().unwrap())
            .product::<f64>();
    }
    let s: f64 = server_prob.iter().sum();
    for serv in server_prob {
        println!("{:.12}", serv / s)
    }
}
