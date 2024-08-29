use std::{
    collections::{hash_map::Entry, HashMap},
    io::{self, BufWriter, Write},
};

use lib::read_input;

// TODO: refactor this mess
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = read_input()?;
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());
    let store_item_cnt = lines.next().unwrap().parse()?;
    let item_costs = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .take(store_item_cnt)
        .flat_map(str::parse)
        .collect::<Vec<usize>>();
    let combo_cost: usize = lines.next().unwrap().parse()?;
    let combo_items_count = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| (x.parse().unwrap(), 0))
        .collect::<HashMap<usize, u16>>();
    let buy_cnt = lines.next().unwrap().parse()?;
    let (cost_no_combo, leftovers) =
        lines.next().unwrap().split_ascii_whitespace().take(buy_cnt).flat_map(str::parse).fold(
            (0_usize, combo_items_count),
            |(mut c, mut acc), item_idx: usize| {
                match acc.entry(item_idx) {
                    // item in combo list - increase counter
                    Entry::Occupied(mut e) => *e.get_mut() += 1,
                    // item not in combo list, just add cost
                    Entry::Vacant(_) => {
                        c += item_costs[item_idx - 1];
                    }
                };
                (c, acc)
            },
        );
    // leftovers - item idx and count
    let mut z: Vec<(usize, u16)> =
        leftovers.into_iter().map(|(k, v)| (item_costs[k - 1], v)).collect();
    // sort by qty desc
    z.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    let mut optimal_cost = cost_no_combo;
    while !z.is_empty() {
        let cur_min_qty = z[0].1;
        let cur_combo_sum: usize =
            z.iter().filter_map(|&(cost, qty)| (qty > 0).then_some(cost)).sum();
        if cur_combo_sum == 0 {
            break;
        }
        optimal_cost += cur_combo_sum.min(combo_cost) * cur_min_qty as usize;
        for (_c, q) in &mut z {
            *q -= cur_min_qty;
        }
        z.retain(|(_c, q)| *q > 0);
    }
    writeln!(out, "{optimal_cost}")?;
    Ok(())
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
}

// endregion: --- Tests

// region: --- Lib
pub mod lib {
    use std::{
        env,
        fmt::Display,
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
        str::FromStr,
    };

    pub fn read_pair<T1, T2>(s: &str) -> (T1, T2)
    where
        T1: FromStr + Copy,
        T2: FromStr + Copy,
    {
        let mut iter = s.split_ascii_whitespace();
        match (iter.next().map(str::parse), iter.next().map(str::parse)) {
            (Some(Ok(first)), Some(Ok(snd))) => (first, snd),
            _ => unreachable!("input is malformed!"),
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn read_input() -> Result<impl Iterator<Item = String>, Box<dyn std::error::Error>> {
        let local_mode = env::var("CODERUN_LOCAL").is_ok_and(|m| m == "true");
        let path = {
            let mut path = PathBuf::new();
            if local_mode {
                path.push(env::var("CARGO_MANIFEST_DIR")?);
            }
            path.push("input.txt");
            path
        };
        let file = File::open(path)?;
        Ok(BufReader::new(file).lines().map_while(Result::ok))
    }

    pub fn join_into_string<T>(a: &[T]) -> String
    where
        T: ToString,
    {
        let Some((first, suffix)) = a.split_first() else { return String::new() };
        let first_owned = first.to_string();
        suffix.iter().fold(first_owned, |mut a, b| {
            a.push(' ');
            a.push_str(&b.to_string());
            a
        })
    }

    pub trait VecStuff<T> {
        fn print(&self, w: usize);
    }
    impl<T: Display + Clone + Copy + 'static> VecStuff<T> for Vec<T> {
        fn print(&self, w: usize) {
            self.iter().enumerate().take(w).for_each(|(i, e)| {
                if i != 0 {
                    print!(" ");
                }
                print!("{e:5}");
            });
        }
    }
}
// endregion" --- Lib
