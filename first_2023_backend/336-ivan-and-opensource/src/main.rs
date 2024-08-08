use std::{
    collections::HashMap,
    io::{self, BufRead, BufWriter, Write},
};

// TODO: maybe implement as &str variant or change back implementation to  String
struct FileExt {
    path: String,
    extension: String,
}

impl<'a> From<&'a str> for FileExt {
    fn from(value: &'a str) -> Self {
        // there always a dot in the filename by task definition
        if let Some((p, ext)) = value.rsplit_once('.') {
            Self { path: p.into(), extension: ext.into() }
        } else {
            unreachable!("File name must contain at least one dot")
        }
    }
}
struct Blacklist {
    data: Vec<String>,
}

impl Blacklist {
    fn is_blacklisted(&self, file: &str) -> bool {
        // TODO: probably there is better way to implement this
        self.data.iter().any(|bf| file.starts_with(bf))
    }
}

impl From<Vec<String>> for Blacklist {
    fn from(mut value: Vec<String>) -> Self {
        // TODO: already forgot why is there - refactor other parts or remove it
        value.sort_unstable();
        Self { data: value }
    }
}

fn main() {
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());
    let mut lines = io::stdin().lock().lines().map_while(Result::ok);
    let n = lines.next().unwrap().parse().unwrap();
    let bl: Blacklist = lines.by_ref().take(n).collect::<Vec<_>>().into();
    let m = lines.next().unwrap().parse().unwrap();
    let files: Vec<FileExt> = lines
        .by_ref()
        .take(m)
        // only need blacklisted files for future queries
        .filter(|f| bl.is_blacklisted(f))
        .map(|f| f.as_str().into())
        .collect();

    let _q = lines.next();
    let mut ext: HashMap<&str, usize> = HashMap::with_capacity(files.len());
    lines.for_each(|query| {
        files.iter().filter(|f| f.path.starts_with(&query)).for_each(|f| {
            ext.entry(&f.extension).and_modify(|count| *count += 1).or_insert(1);
        });
        let _ = writeln!(out, "{}", ext.len());
        for (e, v) in ext.drain() {
            let _ = writeln!(out, "{e}: {v}");
        }
    });
}
