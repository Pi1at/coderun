use std::{
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(Eq, PartialEq, Hash, Debug)]
enum Occupied {
    Empty,
    Black,
    White,
}

fn main() {
    let mut checkers: HashMap<(isize, isize), Occupied> = HashMap::new();
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();
    let must_be_inverse: Vec<(isize, isize)> = vec![(-1, -1), (1, 1), (-1, 1), (1, -1)];
    let must_be_empty: Vec<(isize, isize)> = vec![(-2, -2), (2, 2), (-2, 2), (2, -2)];

    let nm = line_iter
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .flat_map(|s| s.parse::<isize>())
        .collect::<Vec<_>>();
    let num_whites = line_iter.next().unwrap().unwrap().parse().unwrap();
    for _i in 0..num_whites {
        if let [i, j] = line_iter
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<_>>()[0..2]
        {
            checkers.insert((i, j), Occupied::White);
        }
    }
    let num_blacks = line_iter.next().unwrap().unwrap().parse().unwrap();
    for _i in 0..num_blacks {
        if let [i, j] = line_iter
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<_>>()[0..2]
        {
            checkers.insert((i, j), Occupied::Black);
        }
    }
    let turn = match line_iter.next().unwrap().unwrap().as_str() {
        "black" => Occupied::Black,
        "white" => Occupied::White,
        _ => Occupied::Empty,
    };
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
                    Some(state) if (*state != Occupied::Empty) => false,
                    Some(_) | None => match checkers.get(&inv) {
                        Some(state) if (*state == Occupied::Empty) => false,
                        Some(state) if (*state != turn) => true,
                        _ => false,
                    },
                })
        });
    print!("{}", if res { "Yes" } else { "No" })
}
