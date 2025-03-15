use std::io::{self, BufRead};

// poor man u8 to ascii num representation
// just flexing a bit
fn append_itoa(buf: &mut String, v: u8) {
    if v >= 100 {
        buf.push((v / 100 + b'0') as char);
        buf.push(((v / 10) % 10 + b'0') as char);
    } else if v >= 10 {
        buf.push((v / 10 + b'0') as char);
    }
    buf.push((v % 10 + b'0') as char);
}

fn join_into(p: &[u8], buffer: &mut String) {
    let mut iter = p.iter();
    if let Some(&v) = iter.next() {
        append_itoa(buffer, v);
    }
    for &v in iter {
        buffer.push_str(" + ");
        append_itoa(buffer, v);
    }
}

fn solve(n: u8) -> String {
    fn find_partitions(num: u8, buffer: &mut String) {
        fn find_partitions_inner(
            n: u8,
            max_num: u8,
            current_partition: &mut Vec<u8>,
            buffer: &mut String,
        ) {
            if n == 0 {
                // Build the partition string directly
                join_into(current_partition, buffer);
                buffer.push('\n');
                return;
            }

            for i in 1..=max_num.min(n) {
                current_partition.push(i);
                find_partitions_inner(n - i, i, current_partition, buffer);
                current_partition.pop();
            }
        }
        let mut current_partition = Vec::new();
        find_partitions_inner(num, num, &mut current_partition, buffer);
    }

    // just random guess
    let cap = (7 * (n as usize).pow(4)) / 10;
    let mut res = String::with_capacity(cap);
    find_partitions(n, &mut res);
    res
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = io::stdin().lock().lines().next().unwrap()?.parse().map(solve)?;
    print!("{res}");
    Ok(())
}
use std::io::{self, BufRead};

// poor man u8 to ascii num representation
// just flexing a bit
fn append_itoa(buf: &mut String, v: u8) {
    if v >= 100 {
        buf.push((v / 100 + b'0') as char);
        buf.push(((v / 10) % 10 + b'0') as char);
    } else if v >= 10 {
        buf.push((v / 10 + b'0') as char);
    }
    buf.push((v % 10 + b'0') as char);
}

fn join_into(p: &[u8], buffer: &mut String) {
    let mut iter = p.iter();
    if let Some(&v) = iter.next() {
        append_itoa(buffer, v);
    }
    for &v in iter {
        buffer.push_str(" + ");
        append_itoa(buffer, v);
    }
}

fn solve(n: u8) -> String {
    fn find_partitions(num: u8, buffer: &mut String) {
        fn find_partitions_inner(
            n: u8,
            max_num: u8,
            current_partition: &mut Vec<u8>,
            buffer: &mut String,
        ) {
            if n == 0 {
                // Build the partition string directly
                join_into(current_partition, buffer);
                buffer.push('\n');
                return;
            }

            for i in 1..=max_num.min(n) {
                current_partition.push(i);
                find_partitions_inner(n - i, i, current_partition, buffer);
                current_partition.pop();
            }
        }
        let mut current_partition = Vec::new();
        find_partitions_inner(num, num, &mut current_partition, buffer);
    }

    // just random guess
    let cap = (7 * (n as usize).pow(4)) / 10;
    let mut res = String::with_capacity(cap);
    find_partitions(n, &mut res);
    res
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = io::stdin().lock().lines().next().unwrap()?.parse().map(solve)?;
    print!("{res}");
    Ok(())
}
