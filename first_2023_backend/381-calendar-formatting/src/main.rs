use std::fmt::{self, Display, Formatter};
use std::io::BufRead;
use std::io::{self, BufWriter, Write};

fn from_weekday(s: &str) -> usize {
    match s {
        "Monday" => 0,
        "Tuesday" => 1,
        "Wednesday" => 2,
        "Thursday" => 3,
        "Friday" => 4,
        "Saturday" => 5,
        "Sunday" => 6,
        _ => unreachable!(
            "input must be Monday, Tuesday, Wednesday, Thursday, Friday, Saturday or Sunday"
        ),
    }
}

struct Calendar {
    n_days: usize,
    start_day: usize,
}

impl From<String> for Calendar {
    fn from(value: String) -> Self {
        let mut s = value.split_whitespace();
        Self {
            n_days: s.next().unwrap().parse().unwrap(),
            start_day: from_weekday(s.next().unwrap()),
        }
    }
}

impl Display for Calendar {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // FIXME: allocations UwU
        let s = std::iter::repeat("..".to_string())
            .take(self.start_day)
            .chain((1..=self.n_days).map(|i| format!("{i:.>2}")))
            .collect::<Vec<_>>()
            // Split the calendar days into chunks of 7 and format with spaces.
            .chunks(7)
            .map(|w| w.join(" "))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{s}")
    }
}

fn main() {
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());
    let mut line_iter = io::stdin().lock().lines().map_while(Result::ok);
    let c: Calendar = line_iter.next().unwrap().into();
    drop(line_iter);
    _ = writeln!(out, "{c}");
}
