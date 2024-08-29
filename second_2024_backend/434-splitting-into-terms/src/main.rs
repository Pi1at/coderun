use std::io::BufRead;

/// Generates all partitions of a positive integer `n` using numbers up to `max_num`.
///
/// # Arguments
///
/// * `n` - The integer to partition.
/// * `max_num` - The maximum number allowed in the partition.
/// * `current_partition` - A mutable vector to store the current partition.
/// * `result` - A mutable vector to store all partitions.
fn find_partitions(
    n: usize,
    max_num: usize,
    current_partition: &mut Vec<usize>,
    result: &mut Vec<Vec<usize>>,
) {
    if n == 0 {
        result.push(current_partition.clone());
        return;
    }
    for i in 1..=max_num {
        if i <= n {
            current_partition.push(i);
            find_partitions(n - i, i, current_partition, result);
            current_partition.pop();
        }
    }
}

fn main() {
    let n: usize = std::io::stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();
    let mut result = Vec::new();
    let mut current_partition = Vec::new();
    // n<=40 by definition, should be fine
    find_partitions(n, n, &mut current_partition, &mut result);
    for cur in result {
        let res = cur.iter().map(|&x| x.to_string()).collect::<Box<_>>().join(" + ");
        println!("{res}");
    }
}
