#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
use std::{
    collections::HashMap,
    io::{self, BufRead, BufWriter, Write},
};
fn filter_over_bl(file: &str, blacklist: &[String]) -> Option<(String, String)> {
    blacklist
        .iter()
        .find(|&bf| file.starts_with(bf))
        .map(|_bf| {
            let ext = file.rfind('.').unwrap();
            (String::from(file), String::from(&file[ext..]))
        })
}
fn main() {
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());
    let mut line_iter = io::stdin().lock().lines().flatten();

    let n = line_iter.next().unwrap().parse().unwrap();
    let mut bl = vec![String::new(); n];

    (0..bl.len()).for_each(|i| {
        bl[i] = line_iter.next().unwrap();
    });

    let m = line_iter.next().unwrap().parse().unwrap();
    let mut file = String::new();
    let files = {
        let mut files = Vec::with_capacity(m);

        (0..m).for_each(|_idx| {
            file = line_iter.next().unwrap();
            if let Some((s1, s2)) = filter_over_bl(&file, &bl) {
                files.push((s1, s2));
            }
        });
        files
    };

    let q = line_iter.next().unwrap().parse().unwrap();
    let mut ext: HashMap<&str, usize> = HashMap::with_capacity(files.len());
    for _ in 0..q {
        let query = line_iter.next().unwrap(); //directory where delete file
        files
            .iter()
            .filter(|(f, _)| f.starts_with(&query))
            .for_each(|(_, e)| {
                *ext.entry(e).or_default() += 1;
            });
        let _ = writeln!(out, "{}", ext.len());
        for (e, v) in ext.drain() {
            let _ = writeln!(out, "{e}: {v}");
        }
    }
    drop(line_iter);
}
