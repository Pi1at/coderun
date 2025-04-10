use std::collections::HashSet;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug)]
pub struct Interval {
    pub start: i64,
    pub end: i64,
}

impl FromStr for Interval {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_ascii_whitespace().flat_map(str::parse);
        match (iter.next(), iter.next()) {
            (Some(start), Some(end)) => Ok(Self { start, end }),
            _ => Err("input is malformed!"),
        }
    }
}

fn read_pair<T>(s: &str) -> (T, T)
where T: FromStr + Copy {
    let pair: Vec<T> = s.split_ascii_whitespace().flat_map(str::parse).collect();
    (pair[0], pair[1])
}

//TODO: implement more robust solution
/// assume intervals sorted
fn solve(start: i64, end: i64, blues: &[Interval], reds: &[Interval]) -> i64 {
    let count_blues_on = |left, right| {
        let idx = blues.partition_point(|b| b.end <= left);
        let cnt = blues[idx..]
            .iter()
            .take_while(|blue| blue.start < right)
            .map(|blue| right.min(blue.end) - left.max(blue.start))
            .sum::<i64>();
        assert!(cnt >= 0);
        cnt
    };

    let local_ans =
        |x| reds.iter().map(|red| count_blues_on(x + red.start, x + red.end)).sum::<i64>();
    let blues_start = blues.first().unwrap().start;
    let blues_end = blues.last().unwrap().end;
    let reds_start = reds.first().unwrap().start;
    let reds_end = reds.last().unwrap().end;
    // corner cases
    if (blues_end <= reds_start) || (reds_end <= blues_start) {
        return 0;
    }
    // find min and max shift
    let shift_min = start - reds_start;
    let shift_max = end - reds_end;
    // start + x should be in set
    let cl = |diff: i64, r, l| {
        if diff >= 0_i64 {
            // right shift
            diff.min(r)
        } else {
            // left shift
            diff.max(l)
        }
    };
    let candidates = {
        let interval = shift_min..=shift_max;
        // FIXME: maybe remove some b_ends
        let mut points = reds
            .iter()
            .map(|r| (r.start, r.end))
            .fold(HashSet::<i64>::new(), |mut acc, (rs, re)| {
                for Interval { start: bs, end: be } in blues {
                    // FIXME: not all candidates added
                    //acc.insert((be - rs).min(x_max));
                    let diff = be - rs;
                    if interval.contains(&diff) {
                        acc.insert(diff);
                    }
                    acc.insert(cl(bs - re, shift_min, shift_max));
                    let diff = bs - re;
                    if interval.contains(&diff) {
                        acc.insert(diff);
                    }
                }
                acc
            })
            .into_iter()
            .collect::<Vec<_>>();
        points.sort_unstable();
        points
    };

    let mut v_min = i64::MAX;
    if candidates.is_empty() {
        // need to fix somehow
        for x in shift_min..=shift_max {
            v_min = v_min.min(local_ans(x));
            if v_min == 0 {
                return 0;
            }
        }
    } else {
        for x in shift_min..(*candidates.first().unwrap()) {
            v_min = v_min.min(local_ans(x));
            if v_min == 0 {
                return 0;
            }
        }
        for &x in &candidates {
            v_min = v_min.min(local_ans(x));
            if v_min == 0 {
                return 0;
            }
        }
        for x in (*candidates.last().unwrap())..=shift_max {
            v_min = v_min.min(local_ans(x));
            if v_min == 0 {
                return 0;
            }
        }
    }
    v_min
}

fn main() {
    let mut lines = std::io::stdin().lock().lines().map_while(Result::ok);
    let (n, m) = read_pair(&lines.next().unwrap());
    let (l, r) = read_pair::<i64>(&lines.next().unwrap());
    let blue_intervals: Vec<Interval> = lines.by_ref().take(n).flat_map(|s| s.parse()).collect();
    let red_intervals: Vec<Interval> = lines.by_ref().take(m).flat_map(|s| s.parse()).collect();
    drop(lines);
    // input looks sorted, so we good
    let min_intersection = solve(l, r, &blue_intervals, &red_intervals);
    println!("{min_intersection}");
}

// region:    --- Tests

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            solve(0, 5, &[Interval { start: 0, end: 4 }], &[Interval { start: 1, end: 3 }]),
            1
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            solve(0, 5, &[Interval { start: 0, end: 4 }], &[
                Interval { start: 0, end: 1 },
                Interval { start: 2, end: 5 }
            ]),
            3
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            solve(
                0,
                26,
                &[
                    Interval { start: 0, end: 3 },
                    Interval { start: 4, end: 6 },
                    Interval { start: 8, end: 10 },
                    Interval { start: 11, end: 12 },
                    Interval { start: 13, end: 16 },
                    Interval { start: 19, end: 23 }
                ],
                &[Interval { start: 0, end: 3 }]
            ),
            0
        );
    }
}

// endregion: --- Tests
