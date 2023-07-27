use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn runme(strings: &[&str]) -> (usize, usize, Vec<String>) {
    let mut links: HashMap<(&str, &str), usize> = HashMap::new();
    let mut nodes: HashSet<&str> = HashSet::new();

    strings
        .iter()
        .map(|&x| {
            (0..=x.len() - 3)
                .map(|i| {
                    let n = x.get(i..i + 3).unwrap();
                    nodes.insert(n);
                    n
                })
                .collect::<Vec<_>>()
        })
        .for_each(|s| {
            for v in s.windows(2) {
                if let &[n1, n2] = v {
                    *links.entry((n1, n2)).or_insert(0) += 1;
                }
            }
        });
    let output: Vec<_> = links
        .iter()
        .map(|((id1, id2), &v)| format!("{} {} {}", id1, id2, v))
        .collect();
    (nodes.len(), links.len(), output)
}

fn main() {
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();
    let n = line_iter.next().unwrap().unwrap().parse().unwrap();
    let lines: Vec<_> = line_iter.take(n).flatten().collect();
    let lines: Vec<_> = lines.iter().map(|x| &**x).collect();

    let (v, e, rs) = runme(&lines);
    println!("{}", v);
    println!("{}", e);
    for s in rs {
        println!("{}", s)
    }
}
