use std::{
    collections::HashMap,
    convert::Infallible,
    io::{self, BufRead},
    iter,
    str::FromStr,
};

#[derive(Eq, PartialEq, Hash, Debug, Default, Clone)]
enum Field {
    #[default]
    Empty,
    Black,
    White,
}

impl Field {
    /// Returns `true` if the field is [`Empty`].
    ///
    /// [`Empty`]: Field::Empty
    #[must_use]
    const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

impl FromStr for Field {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "black" => Ok(Self::Black),
            "white" => Ok(Self::White),
            _ => Ok(Self::default()), // Orly
        }
    }
}

fn main() {
    let mut checkers: HashMap<(isize, isize), Field> = HashMap::new();
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines().map_while(Result::ok);
    let must_be_inverse: Vec<(isize, isize)> = vec![(-1, -1), (1, 1), (-1, 1), (1, -1)];
    let must_be_empty: Vec<(isize, isize)> = vec![(-2, -2), (2, 2), (-2, 2), (2, -2)];

    let nm = line_iter.next().unwrap().split_whitespace().flat_map(str::parse).collect::<Vec<_>>();
    let num_whites = line_iter.next().unwrap().parse().unwrap();
    let whites = line_iter.by_ref().take(num_whites).map(|s| {
        let mut inner = s.split_ascii_whitespace().flat_map(str::parse);
        (inner.next().unwrap(), inner.next().unwrap())
    });
    checkers.extend(whites.zip(iter::repeat(Field::White)));
    let num_blacks = line_iter.next().unwrap().parse().unwrap();
    let blacks = line_iter.by_ref().take(num_blacks).map(|s| {
        let mut inner = s.split_ascii_whitespace().flat_map(str::parse);
        (inner.next().unwrap(), inner.next().unwrap())
    });
    checkers.extend(blacks.zip(iter::repeat(Field::Black)));
    let turn = line_iter.next().unwrap().parse().unwrap();
    drop(line_iter);

    let is_valid_coord = |(i, j)| i <= nm[0] && i > 0 && j <= nm[1] && j > 0;
    let res = checkers
        .iter()
        //интересуют только фишки нашего цвета
        .filter(|((_, _), color)| **color == turn)
        //выход после первой подошедшей
        .any(|((i, j), _)| {
            (0..must_be_inverse.len())
                .map(|pos| {
                    (
                        (must_be_empty[pos].0 + i, must_be_empty[pos].1 + j),
                        (must_be_inverse[pos].0 + i, must_be_inverse[pos].1 + j),
                    )
                })
                .filter(|(e, inv)| (is_valid_coord(*e) && is_valid_coord(*inv)))
                .any(|(e, inv)| match checkers.get(&e) {
                    Some(field) if (!field.is_empty()) => false,
                    Some(_) | None => match checkers.get(&inv) {
                        Some(field) if (*field != turn) => true,
                        Some(field) if field.is_empty() => false,
                        _ => false,
                    },
                })
        });
    print!("{}", if res { "Yes" } else { "No" });
}
