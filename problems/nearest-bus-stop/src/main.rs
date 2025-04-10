use std::{
    cmp::Ordering,
    io::{self, BufRead},
};

//TODO: get rid from +1 -1
fn solve(bus: i32, bus_stops: &[i32]) -> usize {
    find_nearest_idx(bus, bus_stops) + 1
}

// assuming unique values in array
fn find_nearest_idx(target: i32, arr: &[i32]) -> usize {
    // basically partition point desugared
    match arr.binary_search_by(|&x| if x < target { Ordering::Less } else { Ordering::Greater }) {
        Ok(idx) => idx,

        // target is greater than all
        Err(idx) if idx == arr.len() => idx - 1,
        // target is min
        Err(idx) if idx == 0 => idx,
        // arr[idx] > target
        // arr[idx-1] < target
        Err(idx) => {
            //compare values at idx and idx-1
            idx - usize::from(target.abs_diff(arr[idx]) >= target.abs_diff(arr[idx - 1]))
        }
    }
}

fn main() {
    let mut lines = io::stdin().lock().lines().map_while(Result::ok);
    let _num_queries: usize =
        lines.next().unwrap().split_whitespace().nth(1).unwrap().parse().unwrap();
    let a: Vec<i32> = lines.next().unwrap().split_whitespace().flat_map(str::parse).collect();

    lines.next().unwrap().split_whitespace().flat_map(str::parse).for_each(|bus| {
        println!("{}", solve(bus, &a));
    });
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(4, &[1, 3, 5]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(1, &[1, 3, 5]), 1);
    }

    #[test]
    fn test_greater_all() {
        assert_eq!(solve(7, &[1, 3, 5]), 3);
    }

    #[test]
    fn test_less_all() {
        assert_eq!(solve(0, &[1, 3, 5]), 1);
    }
}
