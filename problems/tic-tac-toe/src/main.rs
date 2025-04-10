use std::io::BufRead;

use lib::read_pair;

#[allow(clippy::significant_drop_tightening)]
fn main() {
    let mut lines = std::io::stdin().lock().lines().map_while(Result::ok);
    let (rows, cols) = read_pair(&lines.next().unwrap());
    let board: Vec<Vec<u8>> = lines.take(rows).map(|line| line.as_bytes().to_vec()).collect();

    let mut is_win = false;
    'outer: for row in 0..rows {
        for col in 0..cols {
            let v = board[row][col];
            if v == b'.' {
                continue;
            };
            if row + 4 < rows {
                is_win |= (1..=4).all(|dy| board[row + dy][col] == v);
            }
            if col + 4 < cols {
                is_win |= (1..=4).all(|dx| board[row][col + dx] == v);
            }
            if (row + 4 < rows) && (col + 4 < cols) {
                is_win |= (1..=4).all(|d| board[row + d][col + d] == v);
            }
            if (row >= 4) && (col + 4 < cols) {
                is_win |= (1..=4).all(|d| board[row - d][col + d] == v);
            }
            if is_win {
                break 'outer;
            };
        }
    }

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
