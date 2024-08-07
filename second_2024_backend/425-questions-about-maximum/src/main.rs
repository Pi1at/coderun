use domain::{build_action_list, Action, Query};
use lib::{join_into_string, read_pair, sorted_vec::SortedVec};
use std::{
    collections::{BTreeSet, HashMap},
    convert::TryFrom,
    env,
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    iter,
    path::PathBuf,
    time::Instant,
};

use crate::lib::VecStuff;

pub mod domain {
    use std::{fmt::Display, str::FromStr};

    use crate::lib::sorted_vec::SortedVec;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Query {
        pub l: u16,
        pub r: u16,
        pub x: u16,
    }

    impl Display for Query {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {} {}", self.l, self.r, self.x)
        }
    }

    impl FromStr for Query {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut iter = s.split_ascii_whitespace().flat_map(str::parse);
            match (iter.next(), iter.next(), iter.next()) {
                (Some(l), Some(r), Some(x)) => Ok(Self { l, r, x }),
                _ => Err("input is malformed!"),
            }
        }
    }

    #[derive(Default, Debug, Clone)]
    pub struct Action {
        pub add: SortedVec<u16>,
        pub remove: SortedVec<u16>,
    }

    // O(q)
    #[must_use]
    pub fn build_action_list(qs: &[Query], n: usize) -> Vec<Action> {
        let mut actions: Vec<Action> = vec![Action::default(); n + 1];
        (0..qs.len()).for_each(|idx| {
            let Query { l, r, x } = qs[idx];
            actions[l as usize - 1].add.push(x);
            actions[r as usize].remove.push(x);
        });
        actions
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let local_mode = env::var("USER").is_ok_and(|u| u == "pi1at");
    let mut path = PathBuf::new();
    if local_mode {
        path.push(env::current_dir()?);
        path.push("425-questions-about-maximum");
    }
    path.push("input.txt");
    let file = File::open(path).expect("file not found!");
    let mut lines = BufReader::new(file).lines().map_while(Result::ok);
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());

    let (n, q) = read_pair(&lines.next().unwrap());
    let queries: Vec<Query> = lines.by_ref().take(q).flat_map(|s| s.parse()).collect();
    //let test_bag: Vec<_> = queries.iter().map(|q| q.x).collect();
    //solve_bag_2(&test_bag, n);
    //queries.sort_unstable();
    let timer = Instant::now();
    let ans = solve(&queries, n);
    let elapsed = timer.elapsed();
    //writeln!(out, "{}", ans.len())?;
    //writeln!(out, "{}", join_into_string(&ans))?;
    writeln!(out, "{}", elapsed.as_millis())?;
    println!("{}", ans.len());
    //println!("{}", join_into_string(&ans));
    Ok(())
}

fn fill_bag(col: u16, qs: &[Query]) -> Vec<u16> {
    let idx = qs.partition_point(|q| q.r < col);
    let mut qf: Vec<_> = qs[idx..].iter().take_while(|q| q.l <= col).map(|q| q.x).collect();
    qf.sort_unstable();
    qf
}
fn fill_bag_qi(col: u16, qs: &[Query]) -> Vec<usize> {
    let idx = qs.partition_point(|q| q.r < col);
    let mut qf: Vec<_> = qs[idx..]
        .iter()
        .enumerate()
        .take_while(|(_k, q)| q.l <= col)
        .map(|(k, _q)| k + idx)
        .collect();
    qf.sort_unstable();
    qf
}

fn bags(queries: &[Query], n: u16) -> impl Iterator<Item = SortedVec<u16>> {
    let mut action_list = build_action_list(queries, n as usize).into_iter();
    let mut current_qs = SortedVec::<_>::new();
    //let mut cache = HashMap::<SortedVec<_>, Vec<bool>>::new();
    //let mut cnt = 0;
    //let mut cache_hits = 0;
    iter::from_fn(move || {
        if let Some(Action { add, remove }) = action_list.next() {
            // remove and after add
            // FIXME: somehow
            for el in remove.iter() {
                assert!(current_qs.remove_item(el).is_some());
            }
            for el in add.iter() {
                current_qs.push(*el);
            }
            Some(current_qs.clone())
        } else {
            None
        }
    })
}

fn bags_x(queries: &[Query], n: u16) -> impl Iterator<Item = Vec<u16>> + '_ {
    let t = Instant::now();
    let mut aq = vec![vec![0; (n + 1) as usize]; queries.len()];
    for (idx, &Query { l, r, x }) in queries.iter().enumerate() {
        aq[idx][l as usize..=r as usize].fill(x);
    }
    println!("build table in time: {}", t.elapsed().as_millis());
    let mut col = 0;
    iter::from_fn(move || {
        let mut bag = Vec::with_capacity(queries.len());
        loop {
            if col >= (n + 1) as usize {
                return None;
            }
            (0..queries.len()).for_each(|idx| {
                let v = aq[idx][col];
                if v != 0 {
                    bag.push(v);
                }
            });
            col += 1;
            if !bag.is_empty() {
                break;
            }
        }
        Some(bag)
    })
}

/// .
///
/// # Panics
///
/// Panics if register size is bigger then u16 bits.
#[must_use]
pub fn find_subset(bag: &[u16], max_sum: &u16) -> Vec<u16> {
    let register_size = u16::try_from(usize::BITS).expect("register is too big, modify code");
    let max_sum = *max_sum;
    let block_count: usize = ((max_sum + 1) / register_size + 1) as usize;

    let mut prev_vector: Vec<usize> = vec![0; block_count];
    let mut result_vector: Vec<u16> = vec![0; block_count * register_size as usize + 1];

    let exit_mask = 1usize << ((max_sum + 1) % register_size - 1);
    let exit_block = block_count - 1;
    prev_vector[0] = 1usize;

    let mut found = false;
    // it's necessary to increment leght of input set, because algorithm requires one additional
    // zero-row
    let rows_count = bag.len() + 1;
    for row_num in 1_usize..rows_count {
        let current_input_set_value = bag[row_num - 1]; // X[i]

        let mut current_vector: Vec<usize> = Vec::with_capacity(block_count);

        for current_block_num in 0_usize..block_count {
            let prev_vector_block = &prev_vector[current_block_num];
            let mut current_block = *prev_vector_block; // make at first T[i, j] = T[i-1, j]

            if current_block == usize::max_value() {
                // there is no sence to make calculations if all bits in block is 1
                // because the result of calculations would be the same
                current_vector.push(current_block);
                continue;
            };

            let shift_size = current_input_set_value;
            let prev_block_num = (shift_size / register_size) as usize;

            match prev_block_num.cmp(&current_block_num) {
                std::cmp::Ordering::Equal => {
                    let prev_block_shift_position = (shift_size % register_size) as usize;
                    let shifted_block = prev_vector[0] << prev_block_shift_position;
                    current_block |= shifted_block;
                }
                std::cmp::Ordering::Less => {
                    let prev_block_shift_position = (shift_size % register_size) as usize;
                    let prev_block_abs_position = current_block_num - prev_block_num - 1;
                    if prev_block_shift_position == 0 {
                        current_block |= prev_vector[prev_block_abs_position + 1];
                    } else {
                        let mut shifted_block = prev_vector[prev_block_abs_position]
                            >> (register_size as usize - prev_block_shift_position);
                        shifted_block |=
                            prev_vector[prev_block_abs_position + 1] << prev_block_shift_position;
                        current_block |= shifted_block;
                    };
                }
                std::cmp::Ordering::Greater => {}
            }

            // check for bits what became 1 in this iteration
            if current_block ^ prev_vector_block != 0 {
                // means that we found new numbers
                // and now it's necessary to put intermediate calculations into result_vector
                // Unfortunately, I have no idea how to unpack numbers accordingly changed bits
                // more easer.
                let mut check_mask = 1usize;
                let new_bits_block = current_block ^ prev_vector_block;
                if result_vector[current_block_num * register_size as usize] == 0
                    && new_bits_block & check_mask != 0
                {
                    result_vector[current_block_num * register_size as usize] =
                        current_input_set_value;
                };
                for bit_position in 1_usize..register_size as usize {
                    check_mask <<= 1;
                    if new_bits_block & check_mask != 0
                        && result_vector[current_block_num * register_size as usize + bit_position]
                            == 0
                    {
                        result_vector[current_block_num * register_size as usize + bit_position] =
                            current_input_set_value;
                    };
                }
            };

            // to drop unnecessary part
            // current_block = current_block & max_num;

            current_vector.push(current_block);

            if (current_block_num == exit_block) && (current_block & exit_mask > 0) {
                found = true;
            };
        }

        prev_vector = current_vector;
        if found {
            break;
        };
    }

    let mut result_numbers: Vec<u16> = Vec::with_capacity(rows_count / 2);
    if found {
        let mut column_index: usize = max_sum as usize;

        for _i in 0..bag.len() {
            if column_index <= 1 {
                break;
            };
            result_numbers.push(result_vector[column_index]);
            column_index -= result_vector[column_index] as usize;
        }
    }
    result_numbers
}

fn solve_bag(bag: &[u16], n: u16) -> Vec<u16> {
    let mut dp = vec![false; n as usize + 1];
    let t = Instant::now();
    //dp.fill(false);
    dp[0] = true;
    for &idx in bag {
        // для каждого запроса перебираем варианты максимума от n-q.x до 0
        let v = idx; //queries[idx].x
        assert!(((n - v) as usize) < dp.len());
        for x in (0..=(n - v) as usize).rev() {
            let i = x + v as usize;
            // ответ для максимума + текущий макс = ответу для максимума
            dp[i] |= dp[x];
            //dpx[x + v as usize] |= dp[x + v as usize];
        }
    }
    println!("with time: {}, len of bag {}", t.elapsed().as_millis(), bag.len());
    dp.iter()
        .enumerate()
        .skip(1)
        .filter_map(|(k, v)| if *v { u16::try_from(k).ok() } else { None })
        .collect()
}

fn solve_bag_2(bag: &[u16], n: u16) -> Vec<u16> {
    let n = n as usize;
    let mut dp = vec![false; n + 1];
    //let t = Instant::now();
    //dp.fill(false);
    dp[0] = true;
    for &idx in bag {
        // для каждого запроса перебираем варианты максимума от n-q.x до 0
        let v = idx as usize; //queries[idx].x
        dp.copy_within(0..=n - v, v);
        dp[v] = true;
    }
    dbg!(&dp);
    //println!("with time: {}, len of bag {}", t.elapsed().as_millis(), bag.len());
    dp.iter()
        .enumerate()
        .skip(1)
        .filter_map(|(k, v)| if *v { u16::try_from(k).ok() } else { None })
        .collect()
}

// FIXME : redo properly
fn solve(queries: &[Query], max_n: u16) -> Vec<u16> {
    // нужно рассмотретьт все возможные комбинации запросов
    // и для каждой комбинации решить knapsack problem
    // для этого проходим от l_min до r_max и считаем комбинации
    //queries.sort_unstable();

    // O(q)
    //let action_list = build_action_list(queries, n as usize);
    //println!("build actions complete");
    // this is our bag to calculate result
    // need to find a way to remove and add element with good time

    //let mut current_qs = SortedVec::<_>::new();
    let n = max_n as usize;
    let mut dp = vec![false; n + 1];
    let mut dpx = vec![false; n + 1];
    let mut cache = HashMap::<SortedVec<_>, Vec<bool>>::new();
    let mut ans = BTreeSet::<u16>::new();
    let mut cnt = 0;
    let mut cache_hits = 0;
    let t = Instant::now();
    //let mut all_bags = bags_x(queries, n as u16).collect::<Vec<_>>();
    //all_bags[0].print(40);
    //all_bags.sort_unstable_by_key(|a| std::cmp::Reverse(a.len()));
    //println!("collected with time: {}, len of bags {}", t.elapsed().as_millis(), all_bags.len());
    for bag in bags(queries, max_n) {
        if cache.keys().any(|k| k.starts_with(&bag)) {
            cache_hits += 1;
            //println!("Cache hits: {cache_hits}");
        } else {
            // if cache.iter().any(|(k, v)| bag.starts_with(k)) {
            //     println!("Potential hit");
            // }
            let t = Instant::now();
            dp.fill(false);
            dp[0] = true;
            for &idx in bag.iter() {
                // для каждого запроса перебираем варианты максимума от n-q.x до 0
                let v = idx as usize; //queries[idx].x
                dp.copy_within(0..n - v, v);
                dp[v] = true;
            }
            //println!("count: {cnt} with time: {}, len of bag {}", t.elapsed().as_millis(), bag.len());
            cache.insert(bag, dp.clone());
            cache_hits += 1;
            cnt += 1;
        }
        dpx.iter_mut().zip(dp.iter()).for_each(|(f, c)| *f |= *c);
    }
    //println!("Cache hits: {cache_hits}");
    ans.extend((0u16..).zip(dpx.iter()).skip(1).filter_map(
        |(k, v)| {
            if *v {
                Some(k)
            } else {
                None
            }
        },
    ));
    ans.into_iter().collect()
}

// FIXME : redo properly
fn solve_naive(queries: &[Query], n: u16) -> Vec<u16> {
    // нужно рассмотретьт все возможные комбинации запросов
    // и для каждой комбинации решить knapsack problem
    // для этого проходим от l_min до r_max и считаем комбинации
    //queries.sort_unstable();
    let mut aq = vec![vec![0; (n + 1) as usize]; queries.len()];
    for (idx, &Query { l, r, x }) in queries.iter().enumerate() {
        aq[idx][l as usize..=r as usize].fill(x);
    }
    for a in aq {
        a.print(n as usize);
        println!();
    }

    let timer = Instant::now();
    let mut dp = vec![false; n as usize + 1];
    let mut bags = HashMap::<Vec<u16>, Vec<u16>>::new();
    let mut cols = queries.iter().fold(Vec::with_capacity(queries.len() * 2), |mut acc, q| {
        acc.push(q.l);
        acc.push(q.r);
        acc
    });
    cols.sort_unstable();
    cols.dedup();
    //println!("time {}", timer.elapsed().as_millis());
    //dbg!(&cols);
    let mut bb = Vec::new();
    for &col in &cols {
        let bag = fill_bag(col, queries);
        bb.push(bag);
    }
    bb.sort_unstable_by(|a, b| match a.len().cmp(&(b.len())) {
        std::cmp::Ordering::Equal => a.cmp(b),
        v => v,
    });
    bb.dedup();
    //println!("total bags: {}", bb.len());
    let elapsed = timer.elapsed();
    //println!("prep time {}", elapsed.as_secs_f32());
    for bag in bb {
        //let bag = fill_bag(col, queries);
        //if !bags.contains_key(&bag) || !bags.keys().any(|k| k.starts_with(&bag)) {
        if !bags.contains_key(&bag)
            || !bags.keys().any(|k| {
                let idx = k.partition_point(|v| v <= &bag[0]);
                k[idx..].starts_with(&bag[1..])
            })
        {
            dp.fill(false);
            dp[0] = true;
            let dp = dp.as_mut_slice();
            for &i in &bag {
                // для каждого запроса перебираем варианты максимума от n-q.x до 0
                for x in (0..=(n - i) as usize).rev() {
                    // ответ для максимума + текущий макс = ответу для максимума
                    dp[x + i as usize] |= dp[x];
                }
            }
            let ans = dp
                .iter()
                .enumerate()
                .skip(1)
                //.filter_map(|(k, v)| if *v && k <= (n as usize) { Some(k as u16) } else { None })
                .filter(|(_k, v)| **v)
                .map(|(k, _v)| k as u16)
                .collect::<Vec<u16>>();
            bags.insert(bag, ans);
        } else {
            // dbg!(&bags);
            // dbg!(&bag);
            // dbg!(bags.contains_key(&bag), bags.keys().any(|k| k.starts_with(&bag)));
        }
    }
    let mut a = bags.into_values().flatten().collect::<Vec<_>>();
    a.sort_unstable();
    a.dedup();
    a
}

// region:    --- Tests

#[cfg(test)]
mod tests {

    use std::{fs, io::Write, iter, time::Instant};

    use crate::lib::join_into_string;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            solve_naive(
                &[
                    Query { l: 1, r: 3, x: 1 },
                    Query { l: 2, r: 4, x: 2 },
                    Query { l: 3, r: 4, x: 4 },
                ],
                4
            ),
            [1, 2, 3, 4]
        );
    }
    #[test]
    fn test_single_bag() {
        assert_eq!(solve_bag_2(&[2, 3, 1], 10), [1, 2, 3, 4, 5, 6]);
        assert_eq!(solve_bag_2(&[1, 2, 3], 4), [1, 2, 3, 4]);
    }
    #[test]
    fn test_1_x() {
        assert_eq!(
            solve_naive(
                &[
                    Query { l: 1, r: 3, x: 1 },
                    Query { l: 2, r: 4, x: 2 },
                    Query { l: 3, r: 4, x: 4 },
                ],
                4
            ),
            [1, 2, 3, 4]
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            solve_naive(&[Query { l: 1, r: 5, x: 1 }, Query { l: 3, r: 7, x: 2 },], 7),
            [1, 2, 3]
        );
    }
    #[test]
    fn test_2_x() {
        assert_eq!(solve(&[Query { l: 1, r: 5, x: 1 }, Query { l: 3, r: 7, x: 2 },], 7), [1, 2, 3]);
    }
    #[test]
    fn test_3() {
        assert_eq!(
            solve_naive(
                &[
                    Query { l: 1, r: 1, x: 2 },
                    Query { l: 1, r: 1, x: 3 },
                    Query { l: 1, r: 1, x: 6 },
                ],
                10
            ),
            [2, 3, 5, 6, 8, 9]
        );
    }
    #[test]
    fn test_3_x() {
        assert_eq!(
            solve(
                &[
                    Query { l: 1, r: 1, x: 2 },
                    Query { l: 1, r: 1, x: 3 },
                    Query { l: 1, r: 1, x: 6 },
                ],
                10
            ),
            [2, 3, 5, 6, 8, 9]
        );
    }

    fn write_test_data_to_file(n: u16, qs: &[Query]) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(format!("input-{n}-{}.txt", qs.len()))?;
        let mut out = BufWriter::with_capacity(1_000_000, file);
        writeln!(out, "{n} {}", qs.len())?;
        for q in qs {
            writeln!(out, "{q}")?;
        }
        Ok(())
    }

    #[test]
    fn generate_bad_case() -> Result<(), Box<dyn std::error::Error>> {
        use rand::{seq::SliceRandom, Rng};
        let mut rng = rand::thread_rng();
        let n_max = 10000;
        let q_max = 10000;
        loop {
            let current_n = rng.gen_range(9000..=n_max);
            let queries_num = rng.gen_range(9900..=q_max);
            let test_data = iter::repeat_with(|| {
                let l = rng.gen_range(1..=current_n);
                let r = rng.gen_range(l..=current_n);
                let x = rng.gen_range(1..=current_n);
                Query { l, r, x }
            })
            .take(queries_num)
            .collect::<Vec<_>>();

            let timer = Instant::now();
            let ans = join_into_string(&solve(&test_data, current_n));
            let elapsed = timer.elapsed();
            //if elapsed.as_millis() > 100 {
            println!("time: {elapsed:?}, n: {} q: {}", current_n, test_data.len());
            //}
            if elapsed.as_millis() > 1000 {
                write_test_data_to_file(current_n, &test_data)?;
                //println!("{test_data:?}");
                println!("{ans}");
                break;
            }
        }
        Ok(())
    }

    fn build_table(qs: &[Query], n: u16) {
        // qs - на текущий момент нужны только сами значения
        let values: Vec<_> = qs.iter().map(|q| q.x).collect();
        // Пусть d(i,c) максимальная сумма ⩽c, подмножества взятого из 1,…, i элементов.
        //let dp = vec![vec![0;]]
    }
    #[test]
    fn build_and_print() {
        build_table(
            &[Query { l: 1, r: 3, x: 1 }, Query { l: 2, r: 4, x: 2 }, Query { l: 3, r: 4, x: 4 }],
            4,
        );
    }
}

// endregion: --- Tests

// region: --- Lib
pub mod lib {
    use std::{fmt::Display, str::FromStr};

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
    pub fn join_into_string<T>(a: &[T]) -> String
    where
        T: ToString,
    {
        let Some((first, suffix)) = a.split_first() else { return String::new() };
        let first_owned = first.to_string();
        suffix.iter().fold(first_owned, |mut a, b| {
            a.push(' ');
            a.push_str(&b.to_string());
            a
        })
    }

    pub trait VecStuff<T> {
        fn print(&self, w: usize);
    }
    impl<T: Display + Clone + Copy + 'static> VecStuff<T> for Vec<T> {
        fn print(&self, w: usize) {
            self.iter().enumerate().take(w).for_each(|(i, e)| {
                if i != 0 {
                    print!(" ");
                }
                print!("{e:5}");
            });
        }
    }

    pub mod bit_vec {
        use std::fmt::{self};
        use std::fmt::{Debug, Formatter};
        use std::hash::Hash;
        use std::ops::{
            BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Sub, SubAssign,
        };

        //Default + Eq + Hash + BitOrAssign + BitAndAssign + BitXorAssign

        /// A `BitSet` type based on bit vectors.
        #[derive(Clone, Default)]
        pub struct BitSet {
            /// # Invariants
            ///
            /// If `num_bits` is not a multiple of `T::NUM_BITS`, the highest
            /// `T::NUM_BITS - num_bits % T::NUM_BITS` bits of the last block of the set
            /// are all zeros.
            ///
            /// the bit indexed by `num_bits - 1` is always set.
            vec: Vec<usize>,

            /// Number of all bits (set & unset).
            num_bits: usize,
        }

        impl BitSet {
            #[must_use]
            pub const fn new() -> Self {
                Self { vec: vec![], num_bits: 0 }
            }

            /// Creates a new empty `BitSet` with the given capacity for the underlying
            /// bit vector.
            ///
            #[must_use]
            pub fn with_capacity(capacity: usize) -> Self {
                Self { vec: Vec::with_capacity(compute_num_blocks(capacity)), num_bits: 0 }
            }

            /// Returns the capacity of the underlying bit vector.
            #[must_use]
            pub fn capacity(&self) -> usize {
                self.vec.capacity().saturating_mul(usize::BITS as usize)
            }

            /// Reserves capacity for at least `additional` more bits for the underlying bit
            /// vector.
            pub fn reserve(&mut self, additional: usize) {
                let cap = self.num_bits.checked_add(additional).expect("capacity overflow");
                if cap > self.capacity() {
                    let vec_len = self.vec.len();
                    self.vec.reserve(compute_num_blocks(cap) - vec_len);
                }
            }

            /// Reserve capacity for exactly `additional` more bits for the underlying bit
            /// vector.
            pub fn reserve_exact(&mut self, additional: usize) {
                let cap = self.num_bits.checked_add(additional).expect("capacity overflow");
                if cap > self.capacity() {
                    let vec_len = self.vec.len();
                    self.vec.reserve_exact(compute_num_blocks(cap) - vec_len);
                }
            }

            /// Shrinks the capacity of the underlying bit vector as much as possible.
            pub fn shrink_to_fit(&mut self) {
                self.vec.shrink_to_fit();
            }

            fn compact(&mut self) {
                for i in (0..self.vec.len()).rev() {
                    let x = self.vec[i];
                    if x.count_ones() != 0 {
                        self.vec.truncate(i + 1);
                        self.num_bits = (i + 1) * usize::BITS as usize - x.leading_zeros() as usize;
                        return;
                    }
                }
                self.vec.clear();
                self.num_bits = 0;
            }

            /// Iterates over the `BitSet`, producing `usize`s representing the elements
            /// in the set, in ascending order.
            #[must_use]
            pub fn iter(&self) -> Iter {
                Iter::new(self)
            }

            /// Returns the number of elements in the set.
            #[must_use]
            pub fn len(&self) -> usize {
                self.vec.iter().map(|x| x.count_ones() as usize).sum()
            }

            /// Returns whether the set is empty.
            #[must_use]
            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }

            /// Clear the set, removing all elements.
            ///
            /// Note that this method has no effect on the allocated capacity of the
            /// underlying bit vector.
            pub fn clear(&mut self) {
                self.vec.clear();
                self.num_bits = 0;
            }

            /// Returns whether the given `value` is present in the set.
            #[must_use]
            pub fn contains(&self, value: usize) -> bool {
                if value >= self.num_bits {
                    return false;
                }

                self.contains_unchecked(value)
            }

            #[inline]
            fn contains_unchecked(&self, value: usize) -> bool {
                self.vec[value / usize::BITS as usize] & (1 << (value % usize::BITS as usize)) != 0
            }

            /// Adds a value to the set.
            pub fn insert(&mut self, value: usize) -> bool {
                let nblks = compute_num_blocks(value + 1);
                if self.vec.len() < nblks {
                    self.vec.resize(nblks, 0);
                }

                if self.num_bits < value + 1 {
                    self.num_bits = value + 1;
                }

                let present = self.contains_unchecked(value);
                self.vec[value / usize::BITS as usize] |= 1 << (value % usize::BITS as usize);
                !present
            }

            /// Removes a value from the set. Returns whether the value was present in the set.
            pub fn remove(&mut self, value: usize) -> bool {
                if value >= self.num_bits {
                    return false;
                }

                let present = self.contains_unchecked(value);
                self.vec[value / usize::BITS as usize] &= !(1 << (value % usize::BITS as usize));

                if present && value + 1 == self.num_bits {
                    self.compact();
                }

                present
            }

            /// Computes the union of the set and `other`.
            #[must_use]
            pub fn union(&self, other: &Self) -> Self {
                self | other
            }

            /// Computes the union of the set and `other` in place.
            pub fn union_with(&mut self, other: &Self) {
                *self |= other;
            }

            /// Computes the intersection of the set and `other`.
            #[must_use]
            pub fn intersection(&self, other: &Self) -> Self {
                self & other
            }

            /// Computes the intersection of the set and `other` in place.
            pub fn intersect_with(&mut self, other: &Self) {
                *self &= other;
            }

            /// Computes the difference of the set and `other`.
            #[must_use]
            pub fn difference(&self, other: &Self) -> Self {
                self - other
            }

            /// Computes the difference of the set and `other` in place.
            pub fn difference_with(&mut self, other: &Self) {
                *self -= other;
            }

            /// Computes the symmetric difference of the set and `other`.
            #[must_use]
            pub fn symmetric_difference(&self, other: &Self) -> Self {
                self ^ other
            }

            /// Computes the symmetric difference of the set and `other` in place.
            pub fn symmetric_difference_with(&mut self, other: &Self) {
                *self ^= other;
            }

            /// Returns whether the set is a subset of `other`.
            #[must_use]
            pub fn is_subset(&self, other: &Self) -> bool {
                if self.num_bits > other.num_bits {
                    return false;
                }

                let nblks = compute_num_blocks(self.num_bits);
                for i in 0..nblks {
                    if self.vec[i] & !other.vec[i] != 0 {
                        return false;
                    }
                }

                true
            }

            /// Returns whether the set is a proper subset of `other`.
            ///
            #[must_use]
            pub fn is_proper_subset(&self, other: &Self) -> bool {
                if self.num_bits > other.num_bits {
                    return false;
                }

                let nblks1 = compute_num_blocks(self.num_bits);
                let nblks2 = compute_num_blocks(other.num_bits);
                let mut equal = nblks1 == nblks2;

                for i in 0..nblks1 {
                    if self.vec[i] & !other.vec[i] != 0 {
                        return false;
                    }

                    if self.vec[i] != other.vec[i] {
                        equal = false;
                    }
                }

                !equal
            }
        }

        #[inline]
        const fn compute_num_blocks(num_bits: usize) -> usize {
            (num_bits + usize::BITS as usize - 1) / (usize::BITS as usize)
        }

        /// An iterator for `BitSet`.
        ///
        /// This struct is created by the [`iter`] method on [`BitSet`]s.
        ///
        /// [`BitSet`]: struct.BitSet.html
        /// [`iter`]: struct.BitSet.html#method.iter
        pub struct Iter<'a> {
            slice: &'a [usize],
            num_bits: usize,
            index: usize,
            bit: usize,
        }

        impl<'a> Iter<'a> {
            pub(crate) fn new(set: &'a BitSet) -> Self {
                Self { slice: &set.vec, num_bits: set.num_bits, index: 0, bit: 0 }
            }
        }

        impl<'a> Iterator for Iter<'a> {
            type Item = usize;

            fn next(&mut self) -> Option<Self::Item> {
                while self.index * usize::BITS as usize + self.bit < self.num_bits {
                    if let Some(bit) = find_lowest_set_bit(self.slice[self.index], self.bit) {
                        self.bit = bit + 1;
                        return Some(
                            self.index
                                .checked_mul(usize::BITS as usize)
                                .and_then(|x| x.checked_add(bit))
                                .expect("element overflow"),
                        );
                    };
                    self.index += 1;
                    self.bit = 0;
                }
                None
            }
        }

        const fn find_lowest_set_bit(blk: usize, from: usize) -> Option<usize> {
            if from >= usize::BITS as usize {
                return None;
            }
            let x = (blk & !((1 << from) - 1)).trailing_zeros();
            if x == usize::BITS {
                None
            } else {
                Some(x as usize)
            }
        }

        impl Debug for BitSet {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.debug_set().entries(self).finish()
            }
        }

        impl<'a> IntoIterator for &'a BitSet {
            type IntoIter = Iter<'a>;
            type Item = usize;

            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl FromIterator<usize> for BitSet {
            fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
                let mut set = Self::default();
                set.extend(iter);
                set
            }
        }

        impl Extend<usize> for BitSet {
            fn extend<I: IntoIterator<Item = usize>>(&mut self, iter: I) {
                for x in iter {
                    self.insert(x);
                }
            }
        }

        impl PartialEq<BitSet> for BitSet {
            fn eq(&self, other: &BitSet) -> bool {
                if self.num_bits != other.num_bits {
                    return false;
                }

                let nblks = compute_num_blocks(self.num_bits);
                self.vec[..nblks] == other.vec[..nblks]
            }
        }

        impl Eq for BitSet {}

        impl Hash for BitSet {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                let nblks = compute_num_blocks(self.num_bits);
                self.vec[..nblks].hash(state);
            }
        }

        #[inline(always)]
        fn bit_or_vec(lhs: &mut BitSet, rhs_vec: &[usize], rhs_nbits: usize) {
            let nblks = compute_num_blocks(lhs.num_bits.min(rhs_nbits));
            for i in 0..nblks {
                lhs.vec[i] |= rhs_vec[i];
            }
        }

        impl BitOr<BitSet> for BitSet {
            type Output = Self;
            fn bitor(mut self, rhs: BitSet) -> BitSet {
                self |= rhs;
                self
            }
        }
        impl<'a> BitOr<&'a BitSet> for &'a BitSet {
            type Output = BitSet;
            fn bitor(self, rhs: &'a BitSet) -> BitSet {
                {
                    if self.num_bits < rhs.num_bits {
                        let mut lhs = rhs.clone();
                        bit_or_vec(&mut lhs, &self.vec, self.num_bits);
                        lhs
                    } else {
                        let mut lhs = self.clone();
                        bit_or_vec(&mut lhs, &rhs.vec, rhs.num_bits);
                        lhs
                    }
                }
            }
        }
        impl BitOrAssign<BitSet> for BitSet {
            #[allow(unused)]
            fn bitor_assign(&mut self, mut rhs: BitSet) {
                {
                    if self.num_bits < rhs.num_bits {
                        std::mem::swap(self, &mut rhs);
                    }
                    bit_or_vec(self, &rhs.vec, rhs.num_bits);
                }
            }
        }
        impl<'a> BitOrAssign<&'a BitSet> for BitSet {
            fn bitor_assign(&mut self, rhs: &'a BitSet) {
                {
                    if self.num_bits < rhs.num_bits {
                        let rhs_nbits = self.num_bits;
                        self.num_bits = rhs.num_bits;
                        if 0 < 1 {
                            self.vec.extend_from_slice(&rhs.vec[self.vec.len()..]);
                        } else {
                            self.vec.truncate(rhs.vec.len());
                        }
                        bit_or_vec(self, &rhs.vec, rhs_nbits);
                    } else {
                        bit_or_vec(self, &rhs.vec, rhs.num_bits);
                    }
                }
            }
        }

        //   op_impl!{
        //     op = (BitAnd, bitand, &=),
        //     swap_cond = >,
        //     op_assign = (BitAndAssign, bitand_assign),

        #[inline(always)]
        fn bit_and_vec(lhs: &mut BitSet, rhs_vec: &[usize], rhs_nbits: usize) {
            let nblks = compute_num_blocks(lhs.num_bits.min(rhs_nbits));
            for i in 0..nblks {
                lhs.vec[i] &= rhs_vec[i];
            }
            lhs.compact();
        }

        impl BitAnd<BitSet> for BitSet {
            type Output = Self;
            fn bitand(mut self, rhs: BitSet) -> BitSet {
                self &= rhs;
                self
            }
        }
        impl<'a> BitAnd<&'a BitSet> for &'a BitSet {
            type Output = BitSet;
            fn bitand(self, rhs: &'a BitSet) -> BitSet {
                {
                    if self.num_bits > rhs.num_bits {
                        let mut lhs = rhs.clone();
                        bit_and_vec(&mut lhs, &self.vec, self.num_bits);
                        lhs
                    } else {
                        let mut lhs = self.clone();
                        bit_and_vec(&mut lhs, &rhs.vec, rhs.num_bits);
                        lhs
                    }
                }
            }
        }
        impl BitAndAssign<BitSet> for BitSet {
            #[allow(unused)]
            fn bitand_assign(&mut self, mut rhs: BitSet) {
                {
                    if self.num_bits > rhs.num_bits {
                        std::mem::swap(self, &mut rhs);
                    }
                    bit_and_vec(self, &rhs.vec, rhs.num_bits);
                }
            }
        }
        impl<'a> BitAndAssign<&'a BitSet> for BitSet {
            fn bitand_assign(&mut self, rhs: &'a BitSet) {
                {
                    if self.num_bits > rhs.num_bits {
                        let rhs_nbits = self.num_bits;
                        self.num_bits > rhs.num_bits;
                        if 0 > 1 {
                            self.vec.extend_from_slice(&rhs.vec[self.vec.len()..]);
                        } else {
                            self.vec.truncate(rhs.vec.len());
                        }
                        bit_and_vec(self, &rhs.vec, rhs_nbits);
                    } else {
                        bit_and_vec(self, &rhs.vec, rhs.num_bits);
                    }
                }
            }
        }
        // <

        #[inline(always)]
        fn bit_xor_vec(lhs: &mut BitSet, rhs_vec: &[usize], rhs_nbits: usize) {
            let nblks = compute_num_blocks(lhs.num_bits.min(rhs_nbits));
            for i in 0..nblks {
                lhs.vec[i] ^= rhs_vec[i];
            }
            lhs.compact();
        }
        impl BitXor<BitSet> for BitSet {
            type Output = Self;
            fn bitxor(mut self, rhs: BitSet) -> BitSet {
                self &= rhs;
                self
            }
        }
        impl<'a> BitXor<&'a BitSet> for &'a BitSet {
            type Output = BitSet;
            fn bitxor(self, rhs: &'a BitSet) -> BitSet {
                {
                    if self.num_bits < rhs.num_bits {
                        let mut lhs = rhs.clone();
                        bit_and_vec(&mut lhs, &self.vec, self.num_bits);
                        lhs
                    } else {
                        let mut lhs = self.clone();
                        bit_and_vec(&mut lhs, &rhs.vec, rhs.num_bits);
                        lhs
                    }
                }
            }
        }
        impl BitXorAssign<BitSet> for BitSet {
            #[allow(unused)]
            fn bitxor_assign(&mut self, mut rhs: BitSet) {
                {
                    if self.num_bits < rhs.num_bits {
                        std::mem::swap(self, &mut rhs);
                    }
                    bit_and_vec(self, &rhs.vec, rhs.num_bits);
                }
            }
        }
        impl<'a> BitXorAssign<&'a BitSet> for BitSet {
            fn bitxor_assign(&mut self, rhs: &'a BitSet) {
                {
                    if self.num_bits < rhs.num_bits {
                        let rhs_nbits = self.num_bits;
                        self.num_bits > rhs.num_bits;
                        if 0 < 1 {
                            self.vec.extend_from_slice(&rhs.vec[self.vec.len()..]);
                        } else {
                            self.vec.truncate(rhs.vec.len());
                        }
                        bit_and_vec(self, &rhs.vec, rhs_nbits);
                    } else {
                        bit_and_vec(self, &rhs.vec, rhs.num_bits);
                    }
                }
            }
        }

        #[inline(always)]
        fn bit_sub_vec(lhs: &mut BitSet, rhs_vec: &[usize], rhs_nbits: usize) {
            let nblks = compute_num_blocks(lhs.num_bits.min(rhs_nbits));
            for i in 0..nblks {
                lhs.vec[i] &= !rhs_vec[i];
            }
            lhs.compact();
        }
        impl Sub<BitSet> for BitSet {
            type Output = BitSet;
            fn sub(mut self, rhs: BitSet) -> BitSet {
                self -= rhs;
                self
            }
        }
        impl<'a> Sub<&'a BitSet> for &'a BitSet {
            type Output = BitSet;
            fn sub(self, rhs: &'a BitSet) -> BitSet {
                {
                    if self.num_bits > rhs.num_bits {
                        let mut lhs = rhs.clone();
                        bit_sub_vec(&mut lhs, &self.vec, self.num_bits);
                        lhs
                    } else {
                        let mut lhs = self.clone();
                        bit_sub_vec(&mut lhs, &rhs.vec, rhs.num_bits);
                        lhs
                    }
                }
            }
        }
        impl SubAssign<BitSet> for BitSet {
            #[allow(unused)]
            fn sub_assign(&mut self, mut rhs: BitSet) {
                {
                    if self.num_bits > rhs.num_bits {
                        std::mem::swap(self, &mut rhs);
                    }
                    bit_sub_vec(self, &rhs.vec, rhs.num_bits);
                }
            }
        }
        impl<'a> SubAssign<&'a BitSet> for BitSet {
            fn sub_assign(&mut self, rhs: &'a BitSet) {
                {
                    if self.num_bits > rhs.num_bits {
                        let rhs_nbits = self.num_bits;
                        self.num_bits = rhs.num_bits;
                        if 0 > 1 {
                            self.vec.extend_from_slice(&rhs.vec[self.vec.len()..]);
                        } else {
                            self.vec.truncate(rhs.vec.len());
                        }
                        bit_sub_vec(self, &rhs.vec, rhs_nbits);
                    } else {
                        bit_sub_vec(self, &rhs.vec, rhs.num_bits);
                    }
                }
            }
        }
    }
    pub mod sorted_vec {

        use std::hash::{Hash, Hasher};

        /// Forward sorted vector
        #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
        pub struct SortedVec<T: Ord> {
            vec: Vec<T>,
        }

        /// Forward sorted set
        #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
        pub struct SortedSet<T: Ord> {
            set: SortedVec<T>,
        }

        /// Value returned when find_or_insert is used.
        #[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
        pub enum FindOrInsert {
            /// Contains a found index
            Found(usize),

            /// Contains an inserted index
            Inserted(usize),
        }

        /// Converts from the binary_search result type into the FindOrInsert type
        impl From<Result<usize, usize>> for FindOrInsert {
            fn from(result: Result<usize, usize>) -> Self {
                match result {
                    Result::Ok(value) => FindOrInsert::Found(value),
                    Result::Err(value) => FindOrInsert::Inserted(value),
                }
            }
        }

        impl FindOrInsert {
            /// Get the index of the element that was either found or inserted.
            pub const fn index(&self) -> usize {
                match self {
                    Self::Found(value) | Self::Inserted(value) => *value,
                }
            }

            /// If an equivalent element was found in the container, get the value of
            /// its index. Otherwise get None.
            pub const fn found(&self) -> Option<usize> {
                match self {
                    Self::Found(value) => Some(*value),
                    Self::Inserted(_) => None,
                }
            }

            /// If the provided element was inserted into the container, get the value
            /// of its index. Otherwise get None.
            #[must_use]
            pub const fn inserted(&self) -> Option<usize> {
                match self {
                    Self::Found(_) => None,
                    Self::Inserted(value) => Some(*value),
                }
            }

            /// Returns true if the element was found.
            #[must_use]
            pub const fn is_found(&self) -> bool {
                matches!(self, Self::Found(_))
            }

            /// Returns true if the element was inserted.
            #[must_use]
            pub const fn is_inserted(&self) -> bool {
                matches!(self, Self::Inserted(_))
            }
        }

        //
        //  impl SortedVec
        //

        impl<T: Ord> SortedVec<T> {
            #[inline]
            pub const fn new() -> Self {
                Self { vec: Vec::new() }
            }
            #[inline]
            pub fn with_capacity(capacity: usize) -> Self {
                Self { vec: Vec::with_capacity(capacity) }
            }
            /// Uses `sort_unstable()` to sort in place.
            #[inline]
            pub fn from_unsorted(mut vec: Vec<T>) -> Self {
                vec.sort_unstable();
                Self { vec }
            }
            /// Insert an element into sorted position, returning the order index at which
            /// it was placed.
            pub fn insert(&mut self, element: T) -> usize {
                let insert_at = match self.binary_search(&element) {
                    Ok(insert_at) | Err(insert_at) => insert_at,
                };
                self.vec.insert(insert_at, element);
                insert_at
            }
            /// Find the element and return the index with `Ok`, otherwise insert the
            /// element and return the new element index with `Err`.
            pub fn find_or_insert(&mut self, element: T) -> FindOrInsert {
                self.binary_search(&element)
                    .map_err(|insert_at| {
                        self.vec.insert(insert_at, element);
                        insert_at
                    })
                    .into()
            }
            /// Same as insert, except performance is O(1) when the element belongs at the
            /// back of the container. This avoids an O(log(N)) search for inserting
            /// elements at the back.
            #[inline]
            pub fn push(&mut self, element: T) -> usize {
                if let Some(last) = self.vec.last() {
                    if element.cmp(last) == std::cmp::Ordering::Less {
                        self.insert(element)
                    } else {
                        self.vec.push(element);
                        self.vec.len() - 1
                    }
                } else {
                    // If there is no last element then the container must be empty, so we
                    // can simply push the element and return its index, which must be 0.
                    self.vec.push(element);
                    0
                }
            }
            /// Reserves additional capacity in the underlying vector.
            /// See `std::vec::Vec::reserve`.
            #[inline]
            pub fn reserve(&mut self, additional: usize) {
                self.vec.reserve(additional);
            }
            /// Same as `find_or_insert`, except performance is O(1) when the element
            /// belongs at the back of the container.
            pub fn find_or_push(&mut self, element: T) -> FindOrInsert {
                if let Some(last) = self.vec.last() {
                    match element.cmp(last) {
                        std::cmp::Ordering::Less =>
                        // The new element is less than the last element in the container, so we
                        // need to fall back on the regular find_or_insert
                        {
                            self.find_or_insert(element)
                        }
                        std::cmp::Ordering::Equal => FindOrInsert::Found(self.vec.len() - 1),
                        std::cmp::Ordering::Greater => {
                            self.vec.push(element);
                            FindOrInsert::Inserted(self.vec.len() - 1)
                        }
                    }
                } else {
                    // If there is no last element then the container must be empty, so we can
                    // simply push the element and return that it was inserted.
                    self.vec.push(element);
                    FindOrInsert::Inserted(0)
                }
            }
            #[inline]
            pub fn remove_item(&mut self, item: &T) -> Option<T> {
                match self.vec.binary_search(item) {
                    Ok(remove_at) => Some(self.vec.remove(remove_at)),
                    Err(_) => None,
                }
            }
            /// Panics if index is out of bounds
            #[inline]
            pub fn remove_index(&mut self, index: usize) -> T {
                self.vec.remove(index)
            }
            #[inline]
            pub fn pop(&mut self) -> Option<T> {
                self.vec.pop()
            }
            #[inline]
            pub fn clear(&mut self) {
                self.vec.clear();
            }
            #[inline]
            pub fn dedup(&mut self) {
                self.vec.dedup();
            }
            #[inline]
            pub fn dedup_by_key<F, K>(&mut self, key: F)
            where
                F: FnMut(&mut T) -> K,
                K: PartialEq<K>,
            {
                self.vec.dedup_by_key(key);
            }
            #[inline]
            pub fn drain<R>(&mut self, range: R) -> std::vec::Drain<T>
            where
                R: std::ops::RangeBounds<usize>,
            {
                self.vec.drain(range)
            }
            #[inline]
            pub fn retain<F>(&mut self, f: F)
            where
                F: FnMut(&T) -> bool,
            {
                self.vec.retain(f);
            }
            /// NOTE: `to_vec()` is a slice method that is accessible through deref, use
            /// this instead to avoid cloning
            #[inline]
            pub fn into_vec(self) -> Vec<T> {
                self.vec
            }
            /// Apply a closure mutating the sorted vector and use `sort_unstable()`
            /// to re-sort the mutated vector
            pub fn mutate_vec<F, O>(&mut self, f: F) -> O
            where
                F: FnOnce(&mut Vec<T>) -> O,
            {
                let res = f(&mut self.vec);
                self.vec.sort_unstable();
                res
            }
            /// The caller must ensure that the provided vector is already sorted.
            #[inline]
            pub unsafe fn from_sorted(vec: Vec<T>) -> Self {
                Self { vec }
            }
            /// Unsafe access to the underlying vector. The caller must ensure that any
            /// changes to the values in the vector do not impact the ordering of the
            /// elements inside, or else this container will misbehave.
            pub unsafe fn get_unchecked_mut_vec(&mut self) -> &mut Vec<T> {
                &mut self.vec
            }
        }
        impl<T: Ord> Default for SortedVec<T> {
            fn default() -> Self {
                Self::new()
            }
        }
        impl<T: Ord> From<Vec<T>> for SortedVec<T> {
            fn from(unsorted: Vec<T>) -> Self {
                Self::from_unsorted(unsorted)
            }
        }
        impl<T: Ord> std::ops::Deref for SortedVec<T> {
            type Target = Vec<T>;
            fn deref(&self) -> &Vec<T> {
                &self.vec
            }
        }
        impl<T: Ord> Extend<T> for SortedVec<T> {
            fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
                for t in iter {
                    let _ = self.insert(t);
                }
            }
        }
        impl<T: Ord + Hash> Hash for SortedVec<T> {
            fn hash<H: Hasher>(&self, state: &mut H) {
                let v: &Vec<T> = self.as_ref();
                v.hash(state);
            }

            fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
            where
                Self: Sized,
            {
                for piece in data {
                    piece.hash(state);
                }
            }
        }

        //
        //  impl SortedSet
        //

        impl<T: Ord> SortedSet<T> {
            #[inline]
            pub fn new() -> Self {
                SortedSet { set: SortedVec::new() }
            }
            #[inline]
            pub fn with_capacity(capacity: usize) -> Self {
                SortedSet { set: SortedVec::with_capacity(capacity) }
            }
            /// Uses `sort_unstable()` to sort in place and `dedup()` to remove
            /// duplicates.
            #[inline]
            pub fn from_unsorted(vec: Vec<T>) -> Self {
                let mut set = SortedVec::from_unsorted(vec);
                set.dedup();
                SortedSet { set }
            }
            /// Insert an element into sorted position, returning the order index at which
            /// it was placed. If an existing item was found it will be returned.
            #[inline]
            pub fn replace(&mut self, mut element: T) -> (usize, Option<T>) {
                match self.set.binary_search(&element) {
                    Ok(existing_index) => {
                        unsafe {
                            // If binary_search worked correctly, then this must be the index of a
                            // valid element to get from the vector.
                            std::mem::swap(
                                &mut element,
                                self.set.vec.get_unchecked_mut(existing_index),
                            )
                        }
                        (existing_index, Some(element))
                    }
                    Err(insert_index) => {
                        self.set.vec.insert(insert_index, element);
                        (insert_index, None)
                    }
                }
            }
            /// Find the element and return the index with `Ok`, otherwise insert the
            /// element and return the new element index with `Err`.
            #[inline]
            pub fn find_or_insert(&mut self, element: T) -> FindOrInsert {
                self.set.find_or_insert(element)
            }
            /// Same as replace, except performance is O(1) when the element belongs at
            /// the back of the container. This avoids an O(log(N)) search for inserting
            /// elements at the back.
            #[inline]
            pub fn push(&mut self, element: T) -> (usize, Option<T>) {
                if let Some(last) = self.vec.last() {
                    let cmp = element.cmp(last);
                    if cmp == std::cmp::Ordering::Greater {
                        // The new element is greater than the current last element, so we can
                        // simply push it onto the vec.
                        self.set.vec.push(element);
                        return (self.vec.len() - 1, None);
                    } else if cmp == std::cmp::Ordering::Equal {
                        // The new element is equal to the last element, so we can simply return
                        // the last index in the vec and the value that is being replaced.
                        let original = self.set.vec.pop();
                        self.set.vec.push(element);
                        return (self.vec.len() - 1, original);
                    } else {
                        // The new element is less than the last element, so we need to fall
                        // back on the regular insert function.
                        return self.replace(element);
                    }
                } else {
                    // If there is no last element then the container must be empty, so we can
                    // simply push the element and return its index, which must be 0.
                    self.set.vec.push(element);
                    return (0, None);
                }
            }
            /// Reserves additional capacity in the underlying vector.
            /// See std::vec::Vec::reserve.
            #[inline]
            pub fn reserve(&mut self, additional: usize) {
                self.set.reserve(additional);
            }
            /// Same as find_or_insert, except performance is O(1) when the element
            /// belongs at the back of the container.
            pub fn find_or_push(&mut self, element: T) -> FindOrInsert {
                self.set.find_or_insert(element)
            }
            #[inline]
            pub fn remove_item(&mut self, item: &T) -> Option<T> {
                self.set.remove_item(item)
            }
            /// Panics if index is out of bounds
            #[inline]
            pub fn remove_index(&mut self, index: usize) -> T {
                self.set.remove_index(index)
            }
            #[inline]
            pub fn pop(&mut self) -> Option<T> {
                self.set.pop()
            }
            #[inline]
            pub fn clear(&mut self) {
                self.set.clear();
            }
            #[inline]
            pub fn drain<R>(&mut self, range: R) -> std::vec::Drain<T>
            where
                R: std::ops::RangeBounds<usize>,
            {
                self.set.drain(range)
            }
            #[inline]
            pub fn retain<F>(&mut self, f: F)
            where
                F: FnMut(&T) -> bool,
            {
                self.set.retain(f);
            }
            /// NOTE: `to_vec()` is a slice method that is accessible through deref, use
            /// this instead to avoid cloning
            #[inline]
            #[must_use]
            pub fn into_vec(self) -> Vec<T> {
                self.set.into_vec()
            }
            /// Apply a closure mutating the sorted vector and use `sort_unstable()`
            /// to re-sort the mutated vector and `dedup()` to remove any duplicate
            /// values
            pub fn mutate_vec<F, O>(&mut self, f: F) -> O
            where
                F: FnOnce(&mut Vec<T>) -> O,
            {
                let res = self.set.mutate_vec(f);
                self.set.dedup();
                res
            }
            /// The caller must ensure that the provided vector is already sorted and
            /// deduped.
            #[inline]
            #[must_use]
            pub unsafe fn from_sorted(vec: Vec<T>) -> Self {
                let set = unsafe { SortedVec::from_sorted(vec) };
                Self { set }
            }
            /// Unsafe access to the underlying vector. The caller must ensure that any
            /// changes to the values in the vector do not impact the ordering of the
            /// elements inside, or else this container will misbehave.
            pub unsafe fn get_unchecked_mut_vec(&mut self) -> &mut Vec<T> {
                self.set.get_unchecked_mut_vec()
            }
        }
        impl<T: Ord> Default for SortedSet<T> {
            fn default() -> Self {
                Self::new()
            }
        }
        impl<T: Ord> From<Vec<T>> for SortedSet<T> {
            fn from(unsorted: Vec<T>) -> Self {
                Self::from_unsorted(unsorted)
            }
        }
        impl<T: Ord> std::ops::Deref for SortedSet<T> {
            type Target = SortedVec<T>;
            fn deref(&self) -> &SortedVec<T> {
                &self.set
            }
        }
        impl<T: Ord> Extend<T> for SortedSet<T> {
            fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
                for t in iter {
                    let _ = self.find_or_insert(t);
                }
            }
        }
        impl<T: Ord + Hash> Hash for SortedSet<T> {
            fn hash<H: Hasher>(&self, state: &mut H) {
                let v: &Vec<T> = self.as_ref();
                v.hash(state);
            }
        }

        /// Reverse-sorted Containers.
        ///
        /// Use these containers to have the vector sorted in the reverse order of its
        /// usual comparison.
        ///
        /// Note that objects going into the reverse container needs to be wrapped in
        /// `std::cmp::Reverse`.
        ///
        /// # Examples
        ///
        /// ```
        /// use std::cmp::Reverse;
        /// use sorted_vec::ReverseSortedVec;
        ///
        /// let mut vec = ReverseSortedVec::<u64>::new();
        /// vec.insert(Reverse(10));
        /// vec.insert(Reverse(15));
        /// assert_eq!(vec.last().unwrap().0, 10);
        /// ```
        pub type ReverseSortedVec<T> = SortedVec<std::cmp::Reverse<T>>;
        pub type ReverseSortedSet<T> = SortedSet<std::cmp::Reverse<T>>;
    }
}
// endregion" --- Lib
