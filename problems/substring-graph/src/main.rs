use std::collections::{HashMap, HashSet};
use std::io::{self, stdout, BufRead, BufWriter, Error, Write};
use std::iter;
use std::string::String;

fn run_me(strings: &[Box<str>]) -> impl Iterator<Item = String> + '_ {
    let mut links: HashMap<(&str, &str), usize> = HashMap::new();
    let mut nodes: HashSet<&str> = HashSet::new();

    for x in strings {
        let mut prev_node: Option<&str> = None;
        for i in 0..=(x.len() - 3) {
            let n = &x[i..(i + 3)];
            nodes.insert(n);
            if let Some(prev) = prev_node {
                links.entry((prev, n)).and_modify(|count| *count += 1).or_insert(1);
            }
            prev_node = Some(n);
        }
    }
    iter::once(nodes.len().to_string())
        .chain(iter::once(links.len().to_string()))
        .chain(links.into_iter().map(|((id1, id2), v)| format!("{id1} {id2} {v}")))
}

fn main() -> Result<(), Error> {
    // just experimenting with strings
    let mut writer = BufWriter::new(stdout().lock());
    let mut line_iter = io::stdin().lock().lines().map_while(Result::ok);
    let lines = {
        let n = line_iter.next().unwrap().parse().unwrap();
        line_iter.by_ref().take(n).map(String::into_boxed_str).collect::<Box<_>>()
    };
    drop(line_iter);
    // lines.len() = 1..=40000
    // line.len() = 4..=30
    // char is a..=z
    // probably there is better way to transform this, but it is what it is ^_^

    for s in run_me(&lines) {
        writeln!(writer, "{s}")?;
    }
    Ok(())
}
