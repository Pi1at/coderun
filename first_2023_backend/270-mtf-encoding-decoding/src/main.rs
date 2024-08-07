#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![warn(clippy::correctness, clippy::suspicious, clippy::complexity, clippy::perf, clippy::style)]

use std::io::{self, BufRead};

use crate::treap_list::TreapList;

pub mod xor_shift_rng {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    pub struct XorShiftRng {
        seed: u32,
    }

    impl XorShiftRng {
        #[must_use]
        pub fn with_random_seed() -> Self {
            #[allow(clippy::cast_possible_truncation)]
            let seed = RandomState::new().build_hasher().finish() as u32;
            Self::with_seed(seed)
        }

        const fn with_seed(seed: u32) -> Self {
            Self { seed }
        }

        // Pseudorandom number generator from the "Xorshift RNGs" paper by George Marsaglia.
        //
        // https://github.com/matklad/config/blob/b8ea0aad0f86d4575651a390a3c7aefb63229774/templates/snippets/src/lib.rs#L30
        // https://github.com/rust-lang/rust/blob/1.55.0/library/core/src/slice/sort.rs#L559-L573
        pub fn rng(self) -> impl Iterator<Item = u32> {
            let mut random = self.seed;
            std::iter::repeat_with(move || {
                random ^= random << 13;
                random ^= random >> 17;
                random ^= random << 5;
                random
            })
        }
    }
}

pub mod implicit_treap {
    use std::{cmp::Ordering, mem};

    pub type Tree<T> = Option<Box<ImplicitNode<T>>>;

    pub fn merge<T>(l_tree: &mut Tree<T>, r_tree: Tree<T>) {
        match (l_tree.take(), r_tree) {
            (Some(mut l_node), Some(mut r_node)) => {
                if l_node.priority > r_node.priority {
                    merge(&mut l_node.right, Some(r_node));
                    l_node.update();
                    *l_tree = Some(l_node);
                } else {
                    let mut new_tree = Some(l_node);
                    merge(&mut new_tree, r_node.left.take());
                    r_node.left = new_tree;
                    r_node.update();
                    *l_tree = Some(r_node);
                }
            }
            (new_tree, None) | (None, new_tree) => *l_tree = new_tree,
        }
    }

    pub fn split<T>(tree: &mut Tree<T>, index: usize, left_inclusive: bool) -> Tree<T> {
        tree.take().and_then(|mut node| {
            let key = node.get_implicit_key();
            match (index.cmp(&key), left_inclusive) {
                (Ordering::Less, _) | (Ordering::Equal, true) => {
                    let res = split(&mut node.left, index, left_inclusive);
                    *tree = node.left.take();
                    node.left = res;
                    node.update();
                    Some(node)
                }
                _ => {
                    let ret = split(&mut node.right, index - key, left_inclusive);
                    node.update();
                    *tree = Some(node);
                    ret
                }
            }
        })
    }

    /// .
    ///
    /// # Panics
    ///
    /// Panics if index out f (1..=len+1).
    pub fn insert<T>(tree: &mut Tree<T>, index: usize, new_node: ImplicitNode<T>) {
        assert!(1 <= index && index <= len(tree) + 1);
        let right = split(tree, index, true);
        merge(tree, Some(Box::new(new_node)));
        merge(tree, right);
    }

    pub fn remove<T>(tree: &mut Tree<T>, index: usize) -> T {
        assert!(1 <= index && index <= len(tree));
        let new_tree = {
            let node = tree.as_mut().expect("Expected non-empty tree.");
            let key = node.get_implicit_key();
            match index.cmp(&key) {
                Ordering::Less => {
                    let ret = remove(&mut node.left, index);
                    node.update();
                    return ret;
                }
                Ordering::Greater => {
                    let ret = remove(&mut node.right, index - key);
                    node.update();
                    return ret;
                }
                Ordering::Equal => {
                    let ImplicitNode { ref mut left, ref mut right, .. } = &mut **node;
                    merge(left, right.take());
                    left.take()
                }
            }
        };

        mem::replace(tree, new_tree).expect("Expected non-empty tree.").value
    }

    #[must_use]
    pub fn get<T>(tree: &Tree<T>, index: usize) -> Option<&T> {
        tree.as_ref().and_then(|node| {
            let key = node.get_implicit_key();
            match index.cmp(&key) {
                Ordering::Less => get(&node.left, index),
                Ordering::Greater => get(&node.right, index - key),
                Ordering::Equal => Some(&node.value),
            }
        })
    }

    pub fn get_mut<T>(tree: &mut Tree<T>, index: usize) -> Option<&mut T> {
        tree.as_mut().and_then(|node| {
            let key = node.get_implicit_key();
            match index.cmp(&key) {
                Ordering::Less => get_mut(&mut node.left, index),
                Ordering::Greater => get_mut(&mut node.right, index - key),
                Ordering::Equal => Some(&mut node.value),
            }
        })
    }

    #[must_use]
    pub fn len<T>(tree: &Tree<T>) -> usize {
        tree.as_ref().map_or(0, |node| node.len())
    }

    pub fn index_of<T>(tree: &Tree<T>, v: &T) -> usize
    where
        T: PartialOrd,
    {
        let mut node = tree.as_deref();
        let mut index = 0;

        while let Some(n) = node {
            if v < &n.value {
                node = n.left.as_deref();
            } else {
                index += n.get_implicit_key();
                node = n.right.as_deref();
            }
        }
        index
    }
    /// A struct representing an internal node of an implicit treap.
    #[derive(Debug)]
    pub struct ImplicitNode<T> {
        pub(crate) value: T,
        pub(crate) priority: u32,
        pub(crate) len: usize,
        pub(crate) left: Tree<T>,
        pub(crate) right: Tree<T>,
    }

    impl<T> ImplicitNode<T> {
        pub const fn new(value: T, priority: u32) -> Self {
            Self { value, priority, len: 1, left: None, right: None }
        }

        pub const fn len(&self) -> usize {
            self.len
        }

        fn update(&mut self) {
            let Self { ref mut len, ref left, ref right, .. } = self;
            *len = 1;
            if let Some(ref left_node) = left {
                *len += left_node.len;
            }
            if let Some(ref right_node) = right {
                *len += right_node.len;
            }
        }

        pub fn get_implicit_key(&self) -> usize {
            self.left.as_ref().map_or(1, |left_node| left_node.len() + 1)
        }
    }
}

pub mod treap_list {
    use crate::{
        implicit_treap::{self, ImplicitNode},
        xor_shift_rng::XorShiftRng,
    };

    pub struct TreapList<T> {
        pub tree: implicit_treap::Tree<T>,
        rng: Box<dyn Iterator<Item = u32>>,
    }

    impl<T> TreapList<T>
    where
        T: PartialOrd,
    {
        /// Constructs a new, empty `TreapList<T>`.
        pub fn new() -> Self {
            Self { tree: None, rng: Box::new(XorShiftRng::with_random_seed().rng()) }
        }

        /// Inserts a value into the list at a particular index, shifting elements one position to the right if needed.
        pub fn insert(&mut self, index: usize, value: T) {
            let Self { ref mut tree, ref mut rng } = self;
            implicit_treap::insert(tree, index + 1, ImplicitNode::new(value, rng.next().unwrap()));
        }

        /// Removes a value at a particular index from the list. Returns the value at the index.
        pub fn remove(&mut self, index: usize) -> T {
            implicit_treap::remove(&mut self.tree, index + 1)
        }

        /// Inserts a value at the front of the list.
        pub fn push_front(&mut self, value: T) {
            self.insert(0, value);
        }

        /// Inserts a value at the back of the list.
        pub fn push_back(&mut self, value: T) {
            let index = self.len();
            self.insert(index, value);
        }

        /// Removes a value at the front of the list.
        pub fn pop_front(&mut self) -> T {
            self.remove(0)
        }

        /// Removes a value at the back of the list.
        pub fn pop_back(&mut self) -> T {
            let index = self.len() - 1;
            self.remove(index)
        }

        /// Returns an immutable reference to the value at a particular index. Returns `None` if the index is out of bounds.
        #[must_use]
        pub fn get(&self, index: usize) -> Option<&T> {
            implicit_treap::get(&self.tree, index + 1)
        }

        /// Returns a mutable reference to the value at a particular index. Returns `None` if the index is out of bounds.
        pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
            implicit_treap::get_mut(&mut self.tree, index + 1)
        }

        /// Returns the number of elements in the list.
        #[must_use]
        pub fn len(&self) -> usize {
            implicit_treap::len(&self.tree)
        }

        /// Returns `true` if the list is empty.
        #[must_use]
        pub const fn is_empty(&self) -> bool {
            self.tree.is_none()
        }

        /// Clears the list, removing all values.
        pub fn clear(&mut self) {
            self.tree = None;
        }

        /// Return index of a value in the list.
        pub fn index_of(&mut self, v: &T) -> Option<usize> {
            match implicit_treap::index_of(&self.tree, v) {
                0 => None,
                idx => Some(idx - 1),
            }
        }
    }
}

fn run_me(input: &str, n: usize, m: usize, decrypt: bool) -> String {
    //pretend we start from 0, so -1 to data
    // zero-indexed treap list
    let mut tr = TreapList::<i32>::new();
    let mut ids: Vec<i32> = (0..m as i32).collect();
    for i in 0..m {
        tr.push_back(i as i32);
    }
    let it = input.split_whitespace().flat_map(str::parse::<usize>).map(|x| x - 1);
    if decrypt {
        it.map(|cur| {
            let c = tr.remove(cur);
            tr.push_front(c);
            c + 1
        })
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ")
    } else {
        let mut min_id = 0;
        it.map(|cur| {
            let id = ids[cur];
            tr.index_of(&id).map_or(0, |idx| {
                tr.remove(idx);
                min_id -= 1;
                tr.push_front(min_id);
                ids[cur] = min_id;
                idx + 1
            })
        })
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ")
    }
}

fn main() {
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();

    let nmt = line_iter
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .flat_map(str::parse)
        .take(3)
        .collect::<Vec<usize>>();
    let decrypt = nmt[2] == 2;
    let input = line_iter.next().unwrap().unwrap();
    let result = run_me(&input, nmt[0], nmt[1], decrypt);
    println!("{result}");

    drop(line_iter);
}
#[cfg(test)]
mod tests {

    use {super::*, rand::Rng};

    #[test]
    fn test_1() {
        assert_eq!("2 3 3 3 2 1 1", run_me("2 3 1 2 1 1 1", 7, 3, false).trim_end());
    }

    #[test]
    fn test_2() {
        assert_eq!("2 3 1 2 1 1 1", run_me("2 3 3 3 2 1 1", 7, 3, true).trim_end());
    }

    #[test]
    fn big_n_m() {
        let mut rng = rand::thread_rng();
        // length
        let max_n = 300_000usize;
        let min_n = 1usize;
        // max value
        let max_m = 300_000usize;
        let min_m = 1usize;

        for _ in 0..10 {
            let n = rng.gen_range(min_n..=max_n);
            let m = rng.gen_range(min_m..=max_m);

            println!("len: {n:<10} max: {m:<10}");

            let before = std::time::Instant::now();
            println!("building string");
            let orig = (0..n)
                .map(|_| rng.gen_range(1..=m))
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            //dbg!(before.elapsed());
            //let before = std::time::Instant::now();
            let encrypted = run_me(&orig, n, m, false);
            //dbg!(before.elapsed());
            //let before = std::time::Instant::now();
            let decrypted = run_me(&encrypted, n, m, true);
            dbg!(before.elapsed());
            assert_eq!(orig, decrypted);
        }
    }
}
