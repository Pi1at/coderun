// TODO: find better solution
use std::convert::TryInto;
use std::io::{self, BufRead};

#[inline]
fn median<T: Sized + Ord + Copy>(a: T, b: T, c: T) -> T {
    c.clamp(a.min(b), a.max(b))
}

fn encode_rle(s: impl AsRef<str>) -> Vec<(char, u8)> {
    let s = s.as_ref().chars();
    let mut count = 1;
    let mut encoded = Vec::new();
    let mut iter = s.peekable();
    while let Some(current) = iter.next() {
        if let Some(&next) = iter.peek() {
            if current == next {
                count += 1;
            } else {
                encoded.push((current, count));
                count = 1;
            }
        } else {
            encoded.push((current, count));
        }
    }
    encoded
}

fn solve<T: AsRef<str>>(arr: &[T; 3]) -> String {
    let spell1 = encode_rle(&arr[0]);
    let spell2 = encode_rle(&arr[1]);
    let spell3 = encode_rle(&arr[2]);
    if spell1.len() != spell2.len() || spell2.len() != spell3.len() {
        return String::from("IMPOSSIBLE");
    }
    spell1
        .into_iter()
        .zip(spell2)
        .zip(spell3)
        .try_fold(String::new(), |mut acc, v| match v {
            (((v1, c1), (v2, c2)), (v3, c3)) if (v1 == v2) && (v2 == v3) => {
                (0..median(c1, c2, c3)).for_each(|_| {
                    acc.push(v1);
                });
                Some(acc)
            }
            _ => None,
        })
        .unwrap_or_else(|| String::from("IMPOSSIBLE"))
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock();
    let crystals: Vec<String> = input.lines().take(3).map_while(Result::ok).collect();
    println!("{}", solve(crystals[0..3].try_into().expect("3 spells given")));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(&["aaaza", "aazzaa", "azzza"]), "aazza");
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(&["xy", "xxyy", "yx"]), "IMPOSSIBLE");
    }
}
