// FIXME: overcomplicated solution

use std::{
    fmt::{self, Display, Formatter},
    io::BufWriter,
    io::Write,
    str::FromStr,
};

use lib::read_input;

#[derive(Debug)]
struct LuckyNumber {
    v: Vec<u8>,
    // 100000 digits, so max value is 9*50000
    sum_left: i32,
    sum_right: i32,
}

impl FromStr for LuckyNumber {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = s.len();
        if n & 1 == 1 {
            return Err("must be even");
        };
        let v = s.bytes().map(|b| b - b'0').collect::<Vec<_>>();
        let mut sum_left: i32 = 0;
        let mut sum_right: i32 = 0;
        // don't check for proper chars for now
        for i in 0..n / 2 {
            sum_left += i32::from(v[i]);
            sum_right += i32::from(v[i + n / 2]);
        }
        Ok(Self { v, sum_left, sum_right })
    }
}

impl Display for LuckyNumber {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = self.v.iter().fold(String::new(), |mut acc, b| {
            acc.push((b + b'0') as char);
            acc
        });
        write!(f, "{s}")
    }
}

impl LuckyNumber {
    const fn is_lucky(&self) -> bool {
        (self.sum_left == self.sum_right) && self.sum_left != 0
    }

    // change to minimum valid lucky number with len n
    fn min_lucky(&mut self) {
        let n = self.v.len();
        self.v.fill(0);
        self.v[n - 1] = 1;
        self.v[(n / 2) - 1] = 1;
        self.sum_right = 1;
        self.sum_left = 1;
    }

    fn add(&mut self, a: u32) {
        self.add_to(self.v.len() - 1, a);
    }

    /// add value starting from given position
    fn add_to(&mut self, pos: usize, mut a: u32) {
        assert!(pos < self.v.len());
        for idx in (0..=pos).rev() {
            if idx < self.v.len() / 2 {
                self.sum_left -= i32::from(self.v[idx]);
            } else {
                self.sum_right -= i32::from(self.v[idx]);
            }
            a += u32::from(self.v[idx]);
            let (carry, rem) = (a / 10, (a % 10) as u8);
            self.v[idx] = rem;
            if idx < self.v.len() / 2 {
                self.sum_left += i32::from(rem);
            } else {
                self.sum_right += i32::from(rem);
            }
            if carry == 0 {
                break;
            };
            a = carry;
        }
    }

    fn next_lucky_fast(&mut self) {
        // remebmer - we need next lucky
        self.add(1);
        loop {
            if self.sum_left == 0 {
                self.min_lucky();
                break;
            }

            let ls = self.sum_left;
            let mut rs = 0;
            let n = self.v.len();
            let mut idx = n / 2;
            // calculate right sum that less or equal leftsum
            // starting from left
            while idx < n {
                // add current digit
                let cur = i32::from(self.v[idx]);
                // on this step ls is still greater rs
                if ls < rs + cur {
                    // fill all with zeroes
                    self.v[idx..n].fill(0);
                    self.sum_right = rs;
                    // need add carry to idx-1
                    self.add_to(idx - 1, 1);
                    break;
                }
                rs += cur;
                idx += 1;
            }
            // after this step, idx == first zero on right or n
            let mut diff = self.sum_left - self.sum_right;
            if diff > 0 {
                // left sum > right sum
                // so add difference
                let mut idx = n - 1;
                while diff > 0 {
                    // as we take minimum, add_up is 0..=9
                    let add_up = i32::from(9 - self.v[idx]).min(diff);
                    // add up to 9 or diff < 9
                    self.v[idx] += add_up as u8;
                    // substract added value
                    diff -= add_up;
                    idx -= 1;
                }
            }

            if diff == 0 {
                if self.sum_left == 0 {
                    self.add_to(n / 2 - 1, 1);
                    continue;
                };
                // found it
                break;
            };
        }
    }

    #[allow(dead_code)]
    fn next_lucky_naive(&mut self) {
        if self.is_lucky() {
            self.add(1);
        }
        while !self.is_lucky() {
            self.add(1);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = read_input()?;
    let mut out = BufWriter::with_capacity(1_000_000, std::io::stdout().lock());
    let mut lucky: LuckyNumber = lines.next().map(|s| s.parse()).unwrap()?;
    lucky.next_lucky_fast();
    writeln!(out, "{lucky}")?;
    Ok(())
}

// region:    --- Tests

#[cfg(test)]
mod tests {

    use super::*;

    fn solve_naive(s: impl AsRef<str>) -> String {
        let mut lucky: LuckyNumber = s.as_ref().parse().unwrap();
        lucky.next_lucky_naive();
        lucky.to_string()
    }
    fn solve(s: impl AsRef<str>) -> String {
        let mut lucky: LuckyNumber = s.as_ref().parse().unwrap();
        lucky.next_lucky_fast();
        lucky.to_string()
    }
    #[test]
    fn test_1_target() {
        assert_eq!(solve_naive("1422"), "1423");
    }
    #[test]
    fn test_1() {
        assert_eq!(solve("1422"), "1423");
    }
    #[test]
    fn test_2_target() {
        assert_eq!(solve_naive("0000"), "0101");
    }
    #[test]
    fn test_2() {
        assert_eq!(solve("0000"), "0101");
    }
    #[test]
    fn test_3() {
        let s1 = "69013468258888787777776666666766687877777877776767676766556665756656576665755666665666666666767666566756566665656655556555555565655555465545655565276777788788878878888888888888898899988999989999908999990000011101122122122222223322222322222322223232222343333243443535555454565666667687887878888778999988898988899898999990999909110112112222232434434454444455444455655546555555545455665651111111111111111011211102111111211111122221221121212212132222";
        //let s1 = "39114826037936936582469247025814702580369258036925703582570358136924702691479257036915703581369147036914703681469257002603603703581369147924702581469258137924702481369247024802580368147925813691469157924702580368247925813691479257025803681360147925813691468146924702570358146925702580368146913692570359147925813692479257136914792470358136914703581469247035703580358136814692479146924792479357035804691469147814692580358036803580368136924702570258925702580369146924792470257024703681368247924792570257035814691479247925702570258146914702589258136813691469257035803581368136814693581469146914792470258035813692479369258146924702580379258036814703581470358146925703692470258047925803682470258146025703681570358247925814792581479358137925703582470268146925814692570368257035914792581469258036025703691470369147036924703692470369247036814703691480358147025814692681369257035924703681469368146935803692570359147936814692681469258147925714692470368146025814692470369147026813692570369147925704792580369258136924702581469257036814792581369147025813692570368147035814702580369147925713691470358136925703581369258136925703581369247035814692470258036824702581470258036924702580368146925803681479247036814702581369147924702580469247025803691470268146924702570358136925602580369257925803682470157035813681469258036824702570358136914692470368146925803681469147925813691479147025814692570368146924702580469237925703581479258146925702570368146925803681469247036814703581469247026814692470258036924702581369247036824702580369248036804692470258136924713681469258147925803691460358136924702691479257036814793581369147036925703681570258046925803681570358136025703591470358136925803691570368147036914703681470258036025803692470259147926813602580360247036924703581470358147925814692580369258036925814692581479257146924803692570369147926914702581370258046924713691479358137025813692470369247025814703681470258247025814692470369147925813792570369147935814692570358247025814702581479257035814793581369147925914792570358247935";
        //let s1 = "83838381695814725826825925925926926936926936936047036937936046037037036046046147037047047148047047037138047147037048047148147047147148148147";
        //       "865771979291839739628 517396285083163062941";
        //let s1 = "865771979291839739628517396285083163062941";
        //3581369147925914792570358247935
        println!("{}", s1.len());
        let mut a = LuckyNumber::from_str(s1).unwrap();
        let mut b = LuckyNumber::from_str(s1).unwrap();
        dbg!(a.sum_left - a.sum_right);
        a.next_lucky_fast();
        b.next_lucky_naive();
        if a.to_string() == b.to_string() {
            println!("{s1} => {b}");
        } else {
            println!("expected {b}, but got {a}");
        }
    }

    #[test]
    fn test_all_below_2n() {
        let half_digits = 2;
        let digits = (half_digits as usize) * 2;
        for i in 0..10_u32.pow(half_digits * 2) {
            let s = format!("{i:0digits$}");
            let mut a = LuckyNumber::from_str(&s).unwrap();
            let mut target = LuckyNumber::from_str(&s).unwrap();
            //println!("{s}");
            target.next_lucky_naive();
            a.next_lucky_fast();
            if a.to_string() != target.to_string() {
                println!("with {s} expected {target}, but got {a}");
                break;
            }
        }
    }

    fn rng_iter() -> impl Iterator<Item = u64> {
        use std::hash::{BuildHasher, Hasher, RandomState};
        let mut random = RandomState::new().build_hasher().finish();
        std::iter::repeat_with(move || {
            random ^= random << 13;
            random ^= random >> 17;
            random ^= random << 5;
            random
        })
    }

    fn generate_num_str(digits: usize) -> String {
        let data = rng_iter()
            .flat_map(u64::to_ne_bytes)
            .map(|v| (v % 10) + b'0')
            .skip_while(|&v| v == 0)
            .take(digits)
            .collect();
        // SAFETY: All bytes are valid ASCII digits
        unsafe { String::from_utf8_unchecked(data) }
    }

    #[test]
    fn test_num_str_start_with_non_zero() {
        for i in 1..500 {
            let s = generate_num_str(i);
            if s.as_bytes()[0] == b'0' {
                println!("{i}: {s}");
                unreachable!();
            }
        }
    }

    #[test]
    fn test_measure() {
        let mut ts = Vec::with_capacity(50_000);
        for i in 1..50 {
            let s = generate_num_str(i * 2);
            println!("{}: {s}", i * 2);
            let mut b = LuckyNumber::from_str(&s).unwrap();
            let t = std::time::Instant::now();
            b.next_lucky_fast();
            ts.push(t.elapsed());
        }
        for (idx, t) in ts.iter().enumerate() {
            println!("{}: time {t:?}", idx * 2);
        }
    }

    #[test]
    fn test_x() {
        let mut a = LuckyNumber::from_str("21119090").unwrap();
        a.next_lucky_naive();
        println!("{a}");
    }

    #[test]
    fn test_xx() {
        let mut a = LuckyNumber::from_str("999999").unwrap();
        a.next_lucky_fast();
        println!("{a}");
    }
}

// endregion: --- Tests

// region: --- Lib
pub mod lib {
    use std::{
        env,
        fmt::Display,
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
        str::FromStr,
    };

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

    #[allow(clippy::missing_errors_doc)]
    pub fn read_input() -> Result<impl Iterator<Item = String>, Box<dyn std::error::Error>> {
        let local_mode = env::var("CODERUN_LOCAL").is_ok_and(|m| m == "true");
        let path = {
            let mut path = PathBuf::new();
            if local_mode {
                path.push(env::var("CARGO_MANIFEST_DIR")?);
            }
            path.push("input.txt");
            path
        };
        let file = File::open(path)?;
        Ok(BufReader::new(file).lines().map_while(Result::ok))
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
}
// endregion" --- Lib
