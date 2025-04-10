use std::io::{self, BufRead};

fn main() {
    let mut line_iter = io::stdin().lock().lines().map_while(Result::ok);
    let num_tests: usize = line_iter.next().unwrap().parse().unwrap();

    for _ in 0..num_tests {
        let _num_chairs: usize = line_iter.next().unwrap().parse().unwrap();

        let chairs = {
            let mut chairs = line_iter
                .next()
                .unwrap()
                .split_whitespace()
                .flat_map(str::parse)
                .collect::<Vec<usize>>();
            chairs.sort_unstable();
            chairs
        };
        let min_diff = chairs.windows(2).map(|z| z[0] ^ z[1]).min().unwrap();
        println!("{min_diff}");
    }
    drop(line_iter);
}
