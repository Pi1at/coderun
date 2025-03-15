// 306. День недели
use std::io::{self, BufRead};
use std::println;

fn run_me(s: &[String]) {
    const DAYNUM_TO_STR: [&str; 7] =
        ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

    const MONTH_TO_NUM: [(&str, usize); 12] = [
        ("Apr", 4),
        ("Aug", 8),
        ("Dec", 12),
        ("Feb", 2),
        ("Jan", 1),
        ("Jul", 7),
        ("Jun", 6),
        ("Mar", 3),
        ("May", 5),
        ("Nov", 11),
        ("Oct", 10),
        ("Sep", 9),
    ];
    fn get_month_num(mm: &str) -> Option<usize> {
        MONTH_TO_NUM.iter().find_map(|(m, n)| if m.eq(&mm) { Some(*n) } else { None })
    }

    //w is the day of the week (0 = Sunday,…,6 = Saturday)
    fn dateyear_to_day_of_week(d: &str, m: &str, y: &str) -> usize {
        let year = y.parse::<usize>().unwrap();
        let day = d.parse::<usize>().unwrap();
        let month = get_month_num(&m[..3]).unwrap();

        let a = (14 - month) / 12;
        let y = year - a;
        let m = month + 12 * a - 2;
        let d = day + 31 * m / 12 + y + y / 4 - y / 100 + y / 400;
        d % 7
    }

    for date_year in s {
        let dmy = date_year.split_whitespace().collect::<Vec<_>>();
        let w = dateyear_to_day_of_week(dmy[0], dmy[1], dmy[2]);
        println!("{}", DAYNUM_TO_STR[w]);
    }
}

/*
В каждой строке содержится по одному тестовому заданию.
Каждая строка содержит день dd, название месяца MM и год yy (1980≤y≤2100).
Имя месяца записано на английском языке и начинается с заглавной буквы.
*/
#[allow(clippy::significant_drop_tightening)]
fn main() {
    let stdin = io::stdin();
    let line_iter = stdin.lock().lines();
    let s_dates = line_iter.map_while(Result::ok).collect::<Vec<_>>();
    run_me(&s_dates);
}
