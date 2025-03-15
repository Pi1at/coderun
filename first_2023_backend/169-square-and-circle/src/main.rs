use std::f64::consts::SQRT_2;
use std::io::{self, BufRead};
use std::println;

fn run_me(points: &[(f64, f64)], r: f64) -> f64 {
    points.iter().map(|center| get_intersect_area(center.0, center.1, r)).sum()
}

fn main() {
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines().map_while(Result::ok);

    let (n_dots, r) = {
        let s_nr = line_iter.next().unwrap();
        let mut nr = s_nr.split_whitespace().take(2);
        (nr.next().unwrap().parse().unwrap(), nr.next().unwrap().parse().unwrap())
    };

    let dots = line_iter
        .by_ref()
        .take(n_dots)
        .map(|s| {
            let mut xy = s.split_whitespace().flat_map(str::parse);
            (xy.next().unwrap(), xy.next().unwrap())
        })
        .collect::<Vec<_>>();
    drop(line_iter);
    let area = run_me(&dots, r);
    println!("{area}");
}

#[allow(clippy::suspicious_operation_groupings)]
fn get_circle_line_area(a: f64, b: f64, r: f64) -> f64 {
    // as positive
    let alpha1 = f64::atan(b / a);
    let alpha2 = f64::atan((1.0 - b) / a);
    let phi = f64::acos(a / r);

    let (phi1, phi2) = if phi.is_nan() { (0.0, 0.0) } else { (phi.min(alpha1), phi.min(alpha2)) };
    let t_a = phi1 + phi2;
    0.5 * r
        .powi(2)
        .mul_add(alpha1 + alpha2 - t_a, a.powi(2) * t_a.sin() / (phi1.cos() * phi2.cos()))
}

fn get_intersect_area(cx: f64, cy: f64, r: f64) -> f64 {
    //в любом случае
    if r > SQRT_2 {
        return 1.0;
    };
    let ab = [(1.0 - cx, cy), (1.0 - cy, 1.0 - cx), (cx, 1.0 - cy), (cy, cx)];
    ab.iter().map(|&(a, b)| get_circle_line_area(a, b, r)).sum()
}

#[cfg(test)]
mod test {

    use rand::Rng;

    use super::*;

    #[test]
    fn random_dot() {
        let mut rng = rand::rng();
        let max_dots: usize = rng.random_range(300..=1000);
        let _r = rng.random_range(0.001..=2.0);

        for _cd in 1..=max_dots {
            let r = rng.random_range(0.001..=2.0);
            let a = rng.random_range(0.001..=0.999);
            let b = rng.random_range(0.001..=0.999);
            let _ = get_intersect_area(a, b, r);
        }
        assert_eq!(0, 0);
    }
}
