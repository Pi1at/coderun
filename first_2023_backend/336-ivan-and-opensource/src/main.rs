use std::{
    collections::HashMap,
    io::{self, BufRead, BufWriter, Write},
};

struct FileExt {
    path: String,
    extension: String,
}

impl<'a> From<&'a str> for FileExt {
    fn from(value: &'a str) -> Self {
        let p = value.to_string();
        let ext = value.rfind('.').unwrap_or_default();
        Self { path: p, extension: (&value[ext..]).into() }
    }
}
struct Blacklist {
    data: Vec<String>,
}

impl Blacklist {
    fn new(mut data: Vec<String>) -> Self {
        data.sort_unstable();
        Self { data }
    }
    fn is_blacklisted(&self, file: &str) -> bool {
        self.data.iter().any(|bf| file.starts_with(bf))
    }
}

fn main() {
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());
    let mut line_iter = io::stdin().lock().lines().map_while(Result::ok);
    let n = line_iter.next().unwrap().parse().unwrap();
    let bl = {
        let b: Vec<_> = line_iter.by_ref().take(n).collect();
        Blacklist::new(b)
    };
    let m = line_iter.next().unwrap().parse().unwrap();
    let files: Vec<FileExt> = line_iter
        .by_ref()
        .take(m)
        .filter(|f| bl.is_blacklisted(f))
        .map(|f| f.as_str().into())
        .collect();

    let _q = line_iter.next();
    let mut ext: HashMap<&str, usize> = HashMap::with_capacity(files.len());
    line_iter.for_each(|query| {
        files.iter().filter(|f| f.path.starts_with(&query)).for_each(|f| {
            ext.entry(&f.extension).and_modify(|count| *count += 1).or_insert(1);
        });
        let _ = writeln!(out, "{}", ext.len());
        for (e, v) in ext.drain() {
            let _ = writeln!(out, "{e}: {v}");
        }
    });
}
