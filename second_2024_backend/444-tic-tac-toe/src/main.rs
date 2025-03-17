use std::io::BufRead;
use std::iter::{self};

use domain::{Counter, update_counters};
use lib::read_pair;

pub mod domain {
    use std::iter;
    use std::ops::{Index, IndexMut};

    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Copy, Clone)]
    pub struct Counter {
        // h v d r
        counters: [u8; 4],
    }
    impl Counter {
        #[must_use]
        pub fn new() -> Self { Self::default() }

        #[must_use]
        pub fn max(&self) -> u8 { self.counters.into_iter().max().unwrap_or_default() }
    }
    impl IndexMut<usize> for Counter {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.counters[index] }
    }
    impl Index<usize> for Counter {
        type Output = u8;

        fn index(&self, index: usize) -> &Self::Output { &self.counters[index] }
    }

    pub fn update_counters(grid: &mut [Vec<Counter>], (x_0, y_0): (usize, usize)) {
        let rows = grid.len();
        let cols = grid[0].len();

        iter::zip(1..=4, 1..=4)
            .flat_map(|(dx, dy)| {
                [
                    ((Some(x_0 + dx), y_0), 0),           // horiz
                    ((Some(x_0), y_0 + dy), 1),           // vert
                    ((Some(x_0 + dx), y_0 + dy), 2),      // diag
                    ((x_0.checked_sub(dx), y_0 + dy), 3), // reverse diag
                ]
            })
            .filter_map(|((s_col, row), idx)| s_col.map(|col| ((col, row), idx)))
            .filter(|((x, y), _idx)| *x < cols && *y < rows)
            .for_each(|((col, row), idx)| grid[row][col][idx] += 1);
    }
}

//NOTE: compare with read and after traverse approach
fn main() {
    let mut lines = std::io::stdin().lock().lines().map_while(Result::ok);

    let (rows, cols) = read_pair(&lines.next().unwrap());

    let crosses_board = vec![vec![Counter::default(); cols]; rows];
    let nougths_board = vec![vec![Counter::default(); cols]; rows];
    let mut boards = [crosses_board, nougths_board];

    let mut is_win = false;
    for row in 0..rows {
        for (col, idx) in
            iter::zip(0..cols, lines.next().unwrap().bytes()).filter_map(|(x, v)| match v {
                b'X' => Some((x, 0)),
                b'O' => Some((x, 1)),
                _ => None,
            })
        {
            if boards[idx][row][col].max() == 4 {
                is_win = true;
                break;
            };
            update_counters(&mut boards[idx], (col, row));
        }
        if is_win {
            break;
        }
    }
    drop(lines);
    println!("{}", if is_win { "Yes" } else { "No" });
}

// region: --- Lib
pub mod lib {
    use std::str::FromStr;

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
}
// endregion" --- Lib
