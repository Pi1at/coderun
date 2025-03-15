use std::collections::HashMap;
use std::io::{self, BufRead};
use std::iter;
use std::str::FromStr;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Checker {
    Black,
    White,
}

impl FromStr for Checker {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "black" => Ok(Self::Black),
            "white" => Ok(Self::White),
            _ => Err("input is malformed!"),
        }
    }
}

fn main() {
    let must_be_inverse = [(-1, -1), (1, 1), (-1, 1), (1, -1)];
    let must_be_empty = [(-2, -2), (2, 2), (-2, 2), (2, -2)];

    let (n, m, checkers, turn) = {
        let mut lines = io::stdin().lock().lines().map_while(Result::ok);

        let (n, m) = lines
            .by_ref()
            .map(|s| {
                let mut nm = s.split_ascii_whitespace().flat_map(str::parse);
                (nm.next().unwrap(), nm.next().unwrap())
            })
            .next()
            .unwrap();

        // hashmap with coordinate and color
        let mut checkers: HashMap<(i16, i16), Checker> = HashMap::new();
        let num_whites = lines.next().unwrap().parse().unwrap();
        let whites = lines.by_ref().take(num_whites).map(|s| {
            let mut inner = s.split_ascii_whitespace().flat_map(str::parse);
            (inner.next().unwrap(), inner.next().unwrap())
        });
        checkers.extend(iter::zip(whites, iter::repeat(Checker::White)));
        let num_blacks = lines.next().unwrap().parse().unwrap();
        let blacks = lines.by_ref().take(num_blacks).map(|s| {
            let mut inner = s.split_ascii_whitespace().flat_map(str::parse);
            (inner.next().unwrap(), inner.next().unwrap())
        });
        checkers.extend(iter::zip(blacks, iter::repeat(Checker::Black)));

        let turn = lines.next().unwrap().parse().unwrap();
        drop(lines);

        (n, m, checkers, turn)
    };

    let is_valid_coord = |(i, j)| i <= n && i > 0 && j <= m && j > 0;

    // интересуют только фишки нашего цвета, выход после первой подошедшей
    let res = checkers.iter().filter_map(|(coord, color)| (*color == turn).then_some(coord)).any(
        |(i, j)| {
            iter::zip(must_be_empty, must_be_inverse)
                .map(|(empty, inv)| ((empty.0 + i, empty.1 + j), (inv.0 + i, inv.1 + j)))
                .filter(|&(empty, inv)| is_valid_coord(empty) && is_valid_coord(inv))
                .any(|(empty, inv)| {
                    !checkers.contains_key(&empty)
                        && checkers.get(&inv).is_some_and(|color| *color != turn)
                })
        },
    );
    print!("{}", if res { "Yes" } else { "No" });
}
