use std::collections::HashMap;
use std::io::BufRead;
use std::io::{self, BufWriter, Write};

type CategoryId = usize;
type Position = usize;
type ProductId = usize;

fn diversity_score(categories: impl Iterator<Item = CategoryId>) -> usize {
    let dm = categories.enumerate().fold(
        HashMap::<CategoryId, Vec<Position>>::new(),
        |mut acc, (pos, c_id)| {
            acc.entry(c_id).or_default().push(pos);
            acc
        },
    );

    dm.values()
        .filter_map(|v| v.windows(2).map(|pos| pos[1] - pos[0]).min())
        .min()
        .unwrap_or(dm.len())
}

fn main() {
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());
    let mut line_iter = io::stdin().lock().lines().map_while(Result::ok);
    let n = line_iter.next().unwrap().parse().unwrap();
    let pc = line_iter
        .by_ref()
        .take(n)
        .map(|s| {
            let mut x = s.split_whitespace().flat_map(str::parse::<usize>);
            match (x.next(), x.next()) {
                (Some(p_id), Some(c_id)) => (p_id, c_id),
                _ => panic!("input format validation failed"),
            }
        })
        .collect::<HashMap<ProductId, CategoryId>>();
    let s = line_iter.next().unwrap();
    drop(line_iter);
    let categories = s.split_whitespace().flat_map(str::parse).map(|x| pc[&x]);
    let r = diversity_score(categories);
    _ = writeln!(out, "{r}");
}
