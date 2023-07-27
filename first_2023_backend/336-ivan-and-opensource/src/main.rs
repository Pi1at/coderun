use std::{
    collections::HashMap,
    io::{self, BufRead, BufWriter, Write},
    writeln,
};

fn filter_over_bl(file: &str, bl: &[String]) -> Option<(String, String)> {
    for bf in bl.iter() {
        if file.starts_with(bf) {
            let ext = file.rfind('.').unwrap();
            return Some((file.to_string(), file[ext..].to_string()));
        }
    }
    None
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
    let mut files = Vec::with_capacity(m);

    (0..m).for_each(|_idx| {
        let file = line_iter.next().unwrap();
        if let Some(v) = filter_over_bl(&file, &bl) {
            files.push(v);
        }
    });

    let q = line_iter.next().unwrap().parse().unwrap();
    (0..q).for_each(|_i| {
        let query = line_iter.next().unwrap(); //directory where delete file
        let mut ext = HashMap::new();

        for (file, e) in files.clone() {
            if file.starts_with(&query) {
                *ext.entry(e).or_insert(0) += 1;
            }
        }

        let _ = writeln!(out, "{}", ext.len());
        for (e, v) in ext.drain() {
            let _ = writeln!(out, "{e}: {v}");
        }
    })
}
