use std::{
    io::{self, BufRead},
    println,
};

fn main() {
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();
    let num_tests: usize = line_iter.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..num_tests {
        let _num_chairs: usize = line_iter.next().unwrap().unwrap().parse().unwrap();

        let chairs = {
            let mut chairs = line_iter
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            chairs.sort_unstable();
            chairs
        };

        let m = chairs.windows(2).map(|z| z[0] ^ z[1]).min().unwrap();

        println!("{}", m);
    }
}
