use std::{
    collections::HashMap,
    io::{self, BufRead},
    iter::FromIterator,
};
fn run_me(s: &str, c: &str) -> usize {
    if s.is_empty() || c.is_empty() {
        return 0;
    };
    let s_chars = s.chars().collect::<Vec<_>>();
    let mut window_freq: HashMap<char, isize> = HashMap::from_iter(c.chars().map(|c| (c, 1)));
    let mut min_len: Option<usize> = None;

    let (mut left, mut right, mut count): (usize, usize, usize) = (0, 0, window_freq.len());

    while right < s_chars.len() {
        let end_char = &s_chars[right];
        right += 1;
        if let Some(f) = window_freq.get_mut(end_char) {
            *f -= 1;
            if *f == 0 {
                count -= 1
            }
        } else {
            // well...this was unclear
            count = window_freq.len();
            left = right;
            window_freq = HashMap::from_iter(c.chars().map(|c| (c, 1)));
        };

        if count > 0 {
            continue;
        }
        // found all
        // resize window now

        while count == 0 {
            let start_char = &s_chars[left];
            left += 1;

            if let Some(f) = window_freq.get_mut(start_char) {
                *f += 1;
                if *f > 0 {
                    count += 1
                }
            };
        }
        // update window

        min_len = Some(match min_len {
            Some(x) => x.min(right - left + 1),
            None => right - left + 1,
        })
    }
    min_len.unwrap_or_default()
}

fn main() {
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();
    let s = line_iter.next().unwrap().unwrap();
    let c = line_iter.next().unwrap().unwrap();

    println!("{}", run_me(&s, &c));
}
