use std::io::BufRead;

// n<=40 by definition, should be fine
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
    find_partitions(n, n, &mut current_partition, &mut result);
    for cur in result {
        let res = cur.iter().map(|&x| x.to_string()).collect::<Box<_>>().join(" + ");
        println!("{res}");
    }
}
