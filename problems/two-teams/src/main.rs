use std::io::{self, BufRead};

fn solve(a: usize, b: usize, n: usize) -> bool {
    let team_1_max = a;
    let team_2_min = b / n + usize::from(b % n > 0);
    team_1_max > team_2_min
}

fn main() {
    let mut lines = io::stdin().lock().lines().map_while(Result::ok);
    let team_1: usize = lines.next().unwrap().parse().unwrap();
    let team_2: usize = lines.next().unwrap().parse().unwrap();
    let max_per_student: usize = lines.next().unwrap().parse().unwrap();
    drop(lines);
    let ans = solve(team_1, team_2, max_per_student);
    println!("{}", if ans { "Yes" } else { "No" });
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        assert!(solve(10, 8, 2));
    }

    #[test]
    fn test_2() {
        assert!(!solve(3, 10, 3));
    }

    #[test]
    fn test_3() {
        assert!(!solve(0, 0, 3));
    }
}
