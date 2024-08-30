use std::cmp::Ordering;
use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Task {
    deadline: u32,
    stress: u32,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.stress.cmp(&other.stress) {
            // if stress is equal, then choose by deadline in reverse
            Ordering::Equal => other.deadline.cmp(&self.deadline),
            ord => ord,
        }
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Task {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ti = s.split_ascii_whitespace().flat_map(str::parse);
        match (ti.next(), ti.next()) {
            (Some(deadline), Some(stress)) => Ok(Self { deadline, stress }),
            _ => Err("input is malformed!"),
        }
    }
}

fn solve(tasks: impl (Iterator<Item = String>)) -> usize {
    // TODO: maybe just insert in proper place instead of sorting?
    // let's build task array
    let mut tasks = tasks.flat_map(|s| s.parse()).collect::<Vec<Task>>();
    // and sort it
    tasks.sort_unstable_by(|a, b| b.cmp(a));

    // TODO: implement dsf version and compare
    // free slots for every task,
    #[allow(clippy::cast_possible_truncation)]
    // we dont't need sore task, just calculate stress
    let mut free_slots = (1..=tasks.len() as u32).collect::<VecDeque<_>>();
    let mut last_failed_deadline: Option<u32> = None;
    let mut total_stress: usize = 0;
    for Task { deadline, stress } in tasks {
        // in prev iteration we already failed with deadline smaller than current
        if last_failed_deadline.is_some_and(|v| v < deadline) {
            last_failed_deadline = free_slots.pop_back();
            total_stress += stress as usize;
            continue;
        }
        // idx of first slot > deadline
        let idx = free_slots.partition_point(|&d| d <= deadline);

        if idx > 0 {
            // maybe need to handle idx > len()
            free_slots.remove(idx - 1);
        } else {
            // failed to find good slot for deadline
            last_failed_deadline = free_slots.pop_back();
            total_stress += stress as usize;
        }
    }
    total_stress
}

fn main() {
    let result = solve(io::stdin().lock().lines().skip(1).map_while(Result::ok));
    println!("{result}");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve(["1 3".to_string(), "1 2".to_string(), "3 1".to_string()].into_iter()), 2);
    }
}
