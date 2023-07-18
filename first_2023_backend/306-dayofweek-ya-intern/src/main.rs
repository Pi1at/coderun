// 306. День недели
use std::{
    collections::HashMap,
    io::{self, BufRead},
    println,
};

fn run_me(s: Vec<String>) {
    let mut month_table = HashMap::<&str, usize>::new();
    month_table.insert("Mar", 3);
    month_table.insert("Apr", 4);
    month_table.insert("May", 5);
    month_table.insert("Jun", 6);
    month_table.insert("Jul", 7);
    month_table.insert("Aug", 8);
    month_table.insert("Sep", 9);
    month_table.insert("Oct", 10);
    month_table.insert("Nov", 11);
    month_table.insert("Dec", 12);
    month_table.insert("Jan", 1);
    month_table.insert("Feb", 2);

    let daynum_to_str = vec![
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    //w is the day of the week (0 = Sunday,…,6 = Saturday)
    let dateyear_to_day_of_week = |d: &str, m: &str, y: &str| {
        let year = y.parse::<usize>().unwrap();
        let day = d.parse::<usize>().unwrap();
        let month = month_table.get(&m[..3]).unwrap();

        let a = (14 - month) / 12;
        let y = year - a;
        let m = month + 12 * a - 2;
        let d = day + 31 * m / 12 + y + y / 4 - y / 100 + y / 400;
        d % 7
    };

    for date_year in s {
        let dmy = date_year.split_whitespace().collect::<Vec<_>>();
        let w = dateyear_to_day_of_week(dmy[0], dmy[1], dmy[2]);
        println! {"{}",daynum_to_str[w]};
    }
}

/*
В каждой строке содержится по одному тестовому заданию.
Каждая строка содержит день dd, название месяца MM и год yy (1980≤y≤2100).
Имя месяца записано на английском языке и начинается с заглавной буквы.
*/
fn main() {
    let stdin = io::stdin();
    let line_iter = stdin.lock().lines();
    let s_dates = line_iter.flatten().collect::<Vec<_>>();
    run_me(s_dates);
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use {super::*, core::panic, rand::{seq::SliceRandom, Rng}};
}
