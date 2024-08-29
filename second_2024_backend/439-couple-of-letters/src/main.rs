use std::convert::TryInto;
use std::{
    collections::BTreeMap,
    io::{self, BufRead},
};

fn main() {
    let words = io::stdin().lock().lines().next().unwrap().unwrap();

    println!("{}", solve(&words));
}

fn solve(words: &str) -> String {
    let max_key = words
        .split_ascii_whitespace()
        .filter(|w| w.len() > 1)
        .flat_map(|w| {
            w.as_bytes()
                .windows(2)
                .map(|t| u16::from_be_bytes(t.try_into().expect("it's always two bytes")))
        })
        // TODO: can't remember why BTreeMap is here, fix later
        .fold(BTreeMap::<u16, u8>::new(), |mut acc, v| {
            acc.entry(v).and_modify(|v| *v += 1).or_insert(1);
            acc
        })
        .into_iter()
        .max_by_key(|(_k, v)| *v)
        .unwrap()
        .0
        .to_be_bytes();
    format!("{}{}", max_key[0] as char, max_key[1] as char)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve("ABCABC A"), "BC");
    }

    #[test]
    fn test_2() {
        // AB BC CA AB BC AB
        assert_eq!(solve("ABCABC AB"), "AB");
    }
}
