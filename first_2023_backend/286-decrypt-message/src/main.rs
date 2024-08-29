use std::{
    collections::BinaryHeap,
    io::{self, BufRead, BufWriter, Write},
};

const BASE: u64 = 1_000_000_007;
const MASK_30: u64 = (1 << 30) - 1;
const MASK_31: u64 = (1 << 31) - 1;
const MOD: u64 = (1 << 61) - 1;

const fn modulo(v: u64) -> u64 {
    let v = (v & MOD) + (v >> 61);
    if v >= MOD {
        v - MOD
    } else {
        v
    }
}
const fn mod_mul(a: u64, b: u64) -> u64 {
    let (a_prefix, a_suffix) = (a >> 31, a & MASK_31);
    let (b_prefix, b_suffix) = (b >> 31, b & MASK_31);
    let m = a_suffix * b_prefix + a_prefix * b_suffix;
    modulo(a_prefix * b_prefix * 2 + (m >> 30) + ((m & MASK_30) << 31) + a_suffix * b_suffix)
}

// shifts v on k elements in 'a..=z'
// so if v = b'a' and k= b'z' we got -25 + 26 = 1
const fn rotate_byte(v: u8, k: u8) -> u64 {
    #![allow(clippy::cast_sign_loss)]
    let c = v as isize - k as isize;
    if c < 0 {
        (c + 26) as u64
    } else {
        c as u64
    }
}
#[inline]
fn rotate_byte_with(k: u8) -> impl Fn(u8) -> u64 {
    move |v| rotate_byte(v, k)
}

// 'a..=z' string
fn get_hash(s: &str) -> u64 {
    if s.len() == 1 {
        1
    } else {
        let rot_k = rotate_byte_with(s.as_bytes()[0]);
        s.as_bytes().iter().fold(0, |hash, &b| modulo(mod_mul(hash, BASE) + rot_k(b) + 1))
    }
}

fn build_book_words(book: &str) -> Vec<(u64, &str)> {
    let mut bh = BinaryHeap::new();
    let b = book.as_bytes();
    let mut k = b[0];
    let mut start_idx = 0;
    let mut end_idx = 0;
    let mut m_hash = modulo(mod_mul(0, BASE) + rotate_byte(k, k) + 1);
    (1..b.len()).for_each(|idx| {
        if b[idx] == b' ' {
            end_idx = idx - 1;
            bh.push((m_hash, &book[start_idx..=end_idx]));
            start_idx = idx + 1;
            if start_idx < b.len() {
                k = b[start_idx];
            };
            m_hash = 0;
        } else {
            m_hash = modulo(mod_mul(m_hash, BASE) + rotate_byte(b[idx], k) + 1);
        }
    });
    bh.push((m_hash, &book[start_idx..b.len()]));
    bh.into_sorted_vec()
}

fn main() {
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());
    let mut line_iter = io::stdin().lock().lines();
    let book = line_iter.next().unwrap().unwrap();
    let bs = build_book_words(&book);

    line_iter
        .by_ref()
        .skip(1)
        .flatten()
        .map(|w| get_hash(&w))
        .flat_map(|h| bs.binary_search_by_key(&h, |(k, _v)| *k))
        .for_each(|v| {
            let _ = writeln!(out, "{}", bs[v].1);
        });
    drop(line_iter);
}
