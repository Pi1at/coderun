use std::io::{self, BufRead};

fn main() {
    let mut all_servers = 0_f64;
    let server_prob: Vec<f64> = io::stdin()
        .lock()
        .lines()
        .skip(1)
        .flatten()
        .map(|s| {
            let sp = s.split_whitespace().flat_map(str::parse::<f64>).product();
            all_servers += sp;
            sp
        })
        .collect();

    for serv in server_prob {
        println!("{:.12}", serv / all_servers);
    }
}
