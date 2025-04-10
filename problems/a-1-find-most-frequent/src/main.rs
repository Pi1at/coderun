use std::{
    collections::HashMap,
    io::{self, BufRead},
};
fn run_me(input: &str) -> u64 {
    let (hmap, max_value) = input.split_whitespace().flat_map(str::parse::<u64>).fold(
        (HashMap::new(), 0_u64),
        |(mut map, mv), x| {
            let mv = mv.max(*map.entry(x).and_modify(|count| *count += 1).or_insert(1));
            (map, mv)
        },
    );
    hmap.into_iter().filter(|&(_, v)| v == max_value).max_by_key(|&(k, _)| k).unwrap_or((0, 0)).0
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().nth(1).unwrap().unwrap();
    println!("{}", run_me(&input));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, run_me("4 4 3 2 3 2 3 2 3 2"));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, run_me("4 1 4 3 3"));
    }
    #[test]
    fn test_3() {
        assert_eq!(10, run_me("10 6 10 10 10 10 8 8 10 9"));
    }
}
