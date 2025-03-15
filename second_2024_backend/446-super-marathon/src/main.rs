//TODO: solved, need cleanup
use std::io::{self, BufWriter, Write};

use domain::{minutes_to_day_hour_min, time_to_minutes, DAYNUM_TO_STR};
use lib::read_input;

pub mod domain {

    pub const DAYNUM_TO_STR: [&str; 7] =
        ["Saturday", "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

    #[must_use]
    pub const fn minutes_to_day(t: usize) -> &'static str {
        let num = (t / (24 * 60)) % 7;
        DAYNUM_TO_STR[num]
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn time_to_minutes(s: &str) -> isize {
        match s.split_once(':') {
            Some((h, m)) => h.parse::<isize>().unwrap() * 60 + m.parse::<isize>().unwrap(),
            None => unreachable!("only valid input assumed!"),
        }
    }

    #[must_use]
    pub fn minutes_to_time(minutes: usize) -> String {
        let hours = (minutes / 60) % 24; // Wrap around after 24 hours
        let mins = minutes % 60;
        format!("{hours:02}:{mins:02}")
    }

    #[must_use]
    pub const fn minutes_to_day_hour_min(t: usize) -> (usize, usize, usize) {
        let t = t % (7 * 24 * 60);
        let (day, daymin) = (t / (24 * 60), t % (24 * 60));
        let (h, m) = (daymin / 60, daymin % 60);
        (day, h, m)
    }
    #[must_use]
    pub fn minutes_to_string(m: isize) -> String {
        let (d, h, m) = minutes_to_day_hour_min(m as usize);
        format!("{}\n{:02}:{:02}", DAYNUM_TO_STR[d], h, m)
    }
}

fn solve(st1: &str, st2: &str, t1: &str, t2: &str) -> Option<isize> {
    // t1*x+st1 == t2*y+st2
    // t1*x - t2*y == st2-st1
    // a = t1, b = -t2, c = st2-st1
    // a*x + b*y == c
    // a!=b!=0

    let start_1 = time_to_minutes(st1);
    let start_2 = time_to_minutes(st2);

    let lap_time_1 = time_to_minutes(t1);
    let lap_time_2 = time_to_minutes(t2);
    if lap_time_1 == 0 && lap_time_2 == 0 {
        return (start_1 == start_2).then_some(start_1);
    }

    let c = start_2 - start_1;
    if let Some((g, x0, y0)) = lib::math::find_any_solution(lap_time_1, -lap_time_2, c) {
        //let ans1 = x0 * lap_time_1 + start_1;
        //let ans2 = y0 * lap_time_2 + start_2;

        //assert_eq!(ans1, ans2);
        //dbg!(ans1, ans2);
        // let find first where they met
        // lap_time_2
        let a = lap_time_1 / g;
        let b = -lap_time_2 / g;
        let mut x = x0;
        let mut y = y0;
        let min_x = 0;
        //start_1.max(start_2);
        let min_y = 0;
        // 1 lap per min, 7*24*60 laps max
        let max_x = 14 * 24 * 60 + 1;
        let max_y = 14 * 24 * 60 + 1;
        let sign_a = a.signum();
        let sign_b = b.signum();
        (x, y) = (x + ((min_x - x) / b) * b, y - ((min_x - x) / b) * a);
        if x < min_x {
            (x, y) = (x + sign_b * b, y - sign_b * a);
        }
        if x > max_x {
            return None;
        }
        let lx1 = x;
        (x, y) = (x + ((max_x - x) / b) * b, y - ((max_x - x) / b) * a);
        if x > max_x {
            (x, y) = (x - sign_b * b, y + sign_b * a);
        }
        //let rx1 = x;
        (x, y) = {
            let k = -(min_y - y) / a;
            (x + k * b, y - k * a)
        };
        if y < min_y {
            (x, y) = (x - sign_a * b, y + sign_a * a);
        }
        if y > max_y {
            return None;
        }
        let mut lx2 = x;

        (x, y) = (x + (-(max_y - y) / a) * b, y - (-(max_y - y) / a) * a);
        if y > max_y {
            (x, y) = (x + sign_a * b, y - sign_a * a);
        }
        let mut rx2 = x;

        if lx2 > rx2 {
            std::mem::swap(&mut lx2, &mut rx2);
        }
        let lx = lx1.max(lx2);
        //let rx = rx1.min(rx2);
        x = lx;
        let mut ans = x * lap_time_1 + start_1;
        while ans < start_1.max(start_2) {
            x += b;
            ans = x * lap_time_1 + start_1;
        }
        Some(ans)
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = read_input()?;
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());

    // start time of runner 1
    let st1 = lines.next().unwrap();
    // start time of runner 2
    let st2 = lines.next().unwrap();
    // runner 1 circle time in minutes
    let t1 = lines.next().unwrap();
    // runner 2 circle time in minutes
    let t2 = lines.next().unwrap();
    match solve(&st1, &st2, &t1, &t2) {
        Some(minutes) => {
            #[allow(clippy::cast_sign_loss)]
            let (d, h, m) = minutes_to_day_hour_min(minutes as usize);
            writeln!(out, "{}", DAYNUM_TO_STR[d])?;
            writeln!(out, "{h:02}:{m:02}")?;
        }
        None => writeln!(out, "Never")?,
    }

    Ok(())
}

// region:    --- Tests

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_1() {
        let m = solve("02:20", "13:00", "05:50", "01:00");
        assert_eq!(m, Some(14 * 60));
    }
    #[test]
    fn test_2() {
        let m = solve("14:36", "10:20", "02:00", "05:00");
        assert_eq!(m, None);
    }
}

// endregion: --- Tests

// region: --- Lib
pub mod lib {
    use std::{
        env,
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

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
    pub mod math {

        #[must_use]
        pub const fn gcd(a: isize, b: isize) -> isize {
            let (mut r, mut rem) = (a, b);
            while rem != 0 {
                (r, rem) = (rem, r % rem);
            }
            r
        }

        /// recursive version
        /// ```
        /// let (g,x0,y0) = gcd_ex(a,b)
        /// assert_eq!(a * x0 + b * y0,g);
        /// ```
        #[must_use]
        pub fn gcd_ex_rec(a: isize, b: isize) -> (isize, isize, isize) {
            if a == 0 {
                return (b, 0, 1);
            }
            let (d, x1, y1) = gcd_ex_rec(b % a, a);
            (d, y1 - (b / a) * x1, x1)
        }

        /// ```
        /// let (g,x0,y0) = gcd_ex(a,b)
        /// assert_eq!(a * x0 + b * y0,g);
        /// ```
        #[must_use]
        pub const fn gcd_ex(a: isize, b: isize) -> (isize, isize, isize) {
            let (mut r, mut rem) = (a, b);
            let (mut x0, mut coeff_s) = (1, 0);
            let (mut y0, mut coeff_t) = (0, 1);

            while rem != 0 {
                let quotient = r / rem;
                (rem, r) = (r - quotient * rem, rem);
                (coeff_s, x0) = (x0 - quotient * coeff_s, coeff_s);
                (coeff_t, y0) = (y0 - quotient * coeff_t, coeff_t);
            }
            (r, x0, y0)
        }

        #[must_use]
        pub const fn find_any_solution(
            a: isize,
            b: isize,
            c: isize,
        ) -> Option<(isize, isize, isize)> {
            let (g, x0, y0) = gcd_ex(a.abs(), b.abs());
            if c % g != 0 {
                return None;
            };
            let mut x0 = x0 * c / g;
            let mut y0 = y0 * c / g;
            if a < 0 {
                x0 = -x0;
            };
            if b < 0 {
                y0 = -y0;
            }
            Some((g, x0, y0))
        }

        /// solve series of a*x == b (mod m)
        #[must_use]
        pub fn solve_congruences(a: &[isize], b: &[isize], m: &[isize]) -> Option<(isize, isize)> {
            let mut x = 0;
            let mut mod_val = 1;
            let n = a.len().min(b.len().min(m.len()));
            for i in 0..n {
                let mut r1 = a[i] * mod_val;
                let mut r2 = m[i];
                let mut x1 = 1;
                let mut x2 = 0;
                let mut r = 1;

                while r != 0 {
                    let q = r1 / r2;
                    (x1, x2) = (x2, x1 - q * x2);
                    r = r1 - q * r2;
                    r1 = r2;
                    r2 = r;
                }

                let b_val = b[i] - a[i] * x;
                if b_val % r1 != 0 {
                    return None;
                }

                x += mod_val * b_val * x1 / r1;
                mod_val *= m[i] / r1;
            }

            if x < 0 || x >= mod_val {
                x -= mod_val * (x / mod_val);
            }
            if x < 0 || x >= mod_val {
                x = (x + mod_val) % mod_val;
            }
            Some((x, mod_val))
        }
    }
}
// endregion" --- Lib
//TODO: solved, need cleanup
use std::io::{self, BufWriter, Write};

use domain::{minutes_to_day_hour_min, time_to_minutes, DAYNUM_TO_STR};
use lib::read_input;

pub mod domain {

    pub const DAYNUM_TO_STR: [&str; 7] =
        ["Saturday", "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

    #[must_use]
    pub const fn minutes_to_day(t: usize) -> &'static str {
        let num = (t / (24 * 60)) % 7;
        DAYNUM_TO_STR[num]
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn time_to_minutes(s: &str) -> isize {
        match s.split_once(':') {
            Some((h, m)) => h.parse::<isize>().unwrap() * 60 + m.parse::<isize>().unwrap(),
            None => unreachable!("only valid input assumed!"),
        }
    }

    #[must_use]
    pub fn minutes_to_time(minutes: usize) -> String {
        let hours = (minutes / 60) % 24; // Wrap around after 24 hours
        let mins = minutes % 60;
        format!("{hours:02}:{mins:02}")
    }

    #[must_use]
    pub const fn minutes_to_day_hour_min(t: usize) -> (usize, usize, usize) {
        let t = t % (7 * 24 * 60);
        let (day, daymin) = (t / (24 * 60), t % (24 * 60));
        let (h, m) = (daymin / 60, daymin % 60);
        (day, h, m)
    }
    #[must_use]
    pub fn minutes_to_string(m: isize) -> String {
        let (d, h, m) = minutes_to_day_hour_min(m as usize);
        format!("{}\n{:02}:{:02}", DAYNUM_TO_STR[d], h, m)
    }
}

fn solve(st1: &str, st2: &str, t1: &str, t2: &str) -> Option<isize> {
    // t1*x+st1 == t2*y+st2
    // t1*x - t2*y == st2-st1
    // a = t1, b = -t2, c = st2-st1
    // a*x + b*y == c
    // a!=b!=0

    let start_1 = time_to_minutes(st1);
    let start_2 = time_to_minutes(st2);

    let lap_time_1 = time_to_minutes(t1);
    let lap_time_2 = time_to_minutes(t2);
    if lap_time_1 == 0 && lap_time_2 == 0 {
        return (start_1 == start_2).then_some(start_1);
    }

    let c = start_2 - start_1;
    if let Some((g, x0, y0)) = lib::math::find_any_solution(lap_time_1, -lap_time_2, c) {
        //let ans1 = x0 * lap_time_1 + start_1;
        //let ans2 = y0 * lap_time_2 + start_2;

        //assert_eq!(ans1, ans2);
        //dbg!(ans1, ans2);
        // let find first where they met
        // lap_time_2
        let a = lap_time_1 / g;
        let b = -lap_time_2 / g;
        let mut x = x0;
        let mut y = y0;
        let min_x = 0;
        //start_1.max(start_2);
        let min_y = 0;
        // 1 lap per min, 7*24*60 laps max
        let max_x = 14 * 24 * 60 + 1;
        let max_y = 14 * 24 * 60 + 1;
        let sign_a = a.signum();
        let sign_b = b.signum();
        (x, y) = (x + ((min_x - x) / b) * b, y - ((min_x - x) / b) * a);
        if x < min_x {
            (x, y) = (x + sign_b * b, y - sign_b * a);
        }
        if x > max_x {
            return None;
        }
        let lx1 = x;
        (x, y) = (x + ((max_x - x) / b) * b, y - ((max_x - x) / b) * a);
        if x > max_x {
            (x, y) = (x - sign_b * b, y + sign_b * a);
        }
        //let rx1 = x;
        (x, y) = {
            let k = -(min_y - y) / a;
            (x + k * b, y - k * a)
        };
        if y < min_y {
            (x, y) = (x - sign_a * b, y + sign_a * a);
        }
        if y > max_y {
            return None;
        }
        let mut lx2 = x;

        (x, y) = (x + (-(max_y - y) / a) * b, y - (-(max_y - y) / a) * a);
        if y > max_y {
            (x, y) = (x + sign_a * b, y - sign_a * a);
        }
        let mut rx2 = x;

        if lx2 > rx2 {
            std::mem::swap(&mut lx2, &mut rx2);
        }
        let lx = lx1.max(lx2);
        //let rx = rx1.min(rx2);
        x = lx;
        let mut ans = x * lap_time_1 + start_1;
        while ans < start_1.max(start_2) {
            x += b;
            ans = x * lap_time_1 + start_1;
        }
        Some(ans)
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = read_input()?;
    let mut out = BufWriter::with_capacity(1_000_000, io::stdout().lock());

    // start time of runner 1
    let st1 = lines.next().unwrap();
    // start time of runner 2
    let st2 = lines.next().unwrap();
    // runner 1 circle time in minutes
    let t1 = lines.next().unwrap();
    // runner 2 circle time in minutes
    let t2 = lines.next().unwrap();
    match solve(&st1, &st2, &t1, &t2) {
        Some(minutes) => {
            #[allow(clippy::cast_sign_loss)]
            let (d, h, m) = minutes_to_day_hour_min(minutes as usize);
            writeln!(out, "{}", DAYNUM_TO_STR[d])?;
            writeln!(out, "{h:02}:{m:02}")?;
        }
        None => writeln!(out, "Never")?,
    }

    Ok(())
}

// region:    --- Tests

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_1() {
        let m = solve("02:20", "13:00", "05:50", "01:00");
        assert_eq!(m, Some(14 * 60));
    }
    #[test]
    fn test_2() {
        let m = solve("14:36", "10:20", "02:00", "05:00");
        assert_eq!(m, None);
    }
}

// endregion: --- Tests

// region: --- Lib
pub mod lib {
    use std::{
        env,
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

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
    pub mod math {

        #[must_use]
        pub const fn gcd(a: isize, b: isize) -> isize {
            let (mut r, mut rem) = (a, b);
            while rem != 0 {
                (r, rem) = (rem, r % rem);
            }
            r
        }

        /// recursive version
        /// ```
        /// let (g,x0,y0) = gcd_ex(a,b)
        /// assert_eq!(a * x0 + b * y0,g);
        /// ```
        #[must_use]
        pub fn gcd_ex_rec(a: isize, b: isize) -> (isize, isize, isize) {
            if a == 0 {
                return (b, 0, 1);
            }
            let (d, x1, y1) = gcd_ex_rec(b % a, a);
            (d, y1 - (b / a) * x1, x1)
        }

        /// ```
        /// let (g,x0,y0) = gcd_ex(a,b)
        /// assert_eq!(a * x0 + b * y0,g);
        /// ```
        #[must_use]
        pub const fn gcd_ex(a: isize, b: isize) -> (isize, isize, isize) {
            let (mut r, mut rem) = (a, b);
            let (mut x0, mut coeff_s) = (1, 0);
            let (mut y0, mut coeff_t) = (0, 1);

            while rem != 0 {
                let quotient = r / rem;
                (rem, r) = (r - quotient * rem, rem);
                (coeff_s, x0) = (x0 - quotient * coeff_s, coeff_s);
                (coeff_t, y0) = (y0 - quotient * coeff_t, coeff_t);
            }
            (r, x0, y0)
        }

        #[must_use]
        pub const fn find_any_solution(
            a: isize,
            b: isize,
            c: isize,
        ) -> Option<(isize, isize, isize)> {
            let (g, x0, y0) = gcd_ex(a.abs(), b.abs());
            if c % g != 0 {
                return None;
            };
            let mut x0 = x0 * c / g;
            let mut y0 = y0 * c / g;
            if a < 0 {
                x0 = -x0;
            };
            if b < 0 {
                y0 = -y0;
            }
            Some((g, x0, y0))
        }

        /// solve series of a*x == b (mod m)
        #[must_use]
        pub fn solve_congruences(a: &[isize], b: &[isize], m: &[isize]) -> Option<(isize, isize)> {
            let mut x = 0;
            let mut mod_val = 1;
            let n = a.len().min(b.len().min(m.len()));
            for i in 0..n {
                let mut r1 = a[i] * mod_val;
                let mut r2 = m[i];
                let mut x1 = 1;
                let mut x2 = 0;
                let mut r = 1;

                while r != 0 {
                    let q = r1 / r2;
                    (x1, x2) = (x2, x1 - q * x2);
                    r = r1 - q * r2;
                    r1 = r2;
                    r2 = r;
                }

                let b_val = b[i] - a[i] * x;
                if b_val % r1 != 0 {
                    return None;
                }

                x += mod_val * b_val * x1 / r1;
                mod_val *= m[i] / r1;
            }

            if x < 0 || x >= mod_val {
                x -= mod_val * (x / mod_val);
            }
            if x < 0 || x >= mod_val {
                x = (x + mod_val) % mod_val;
            }
            Some((x, mod_val))
        }
    }
}
// endregion" --- Lib
