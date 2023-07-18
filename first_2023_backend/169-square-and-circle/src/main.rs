// 169. Квадрат и окружность
use std::{
    f64::consts::*,
    io::{self, BufRead},
    println,
};

fn run_me(points: &[(f64, f64)], r: f64) -> f64 {
    points.iter().fold(0.0, |area, center| {
        area + get_intersect_area(center.0, center.1, r)
    })
}

fn main() {
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();

    let nr = line_iter
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .flat_map(|s| s.parse::<f64>())
        .collect::<Vec<_>>();
    let n_dots = nr[0] as usize;
    let r = nr[1];
    let mut dots = Vec::<(f64, f64)>::with_capacity(n_dots);

    for xy in line_iter.take(n_dots) {
        if let [x, y] = xy
            .unwrap()
            .split_whitespace()
            .flat_map(|x| x.parse::<f64>())
            .collect::<Vec<_>>()[0..2]
        {
            dots.push((x, y));
        }
    }

    let area = run_me(&dots, r);
    println!("{}", area);
}

fn get_circle_line_area(a: f64, b: f64, r: f64) -> f64 {
    // as positive
    let alpha1 = f64::atan(b / a);
    let alpha2 = f64::atan((1.0 - b) / a);
    let phi = f64::acos(a / r);
    //print!("> {:.3} {:.3} {:.3}", alpha1, alpha2, phi);

    let (t_a, phi1, phi2) = if phi.is_nan() {
        (0.0, 0.0, 0.0)
    } else {
        (
            phi.min(alpha2) + phi.min(alpha1),
            phi.min(alpha1),
            phi.min(alpha2),
        )
    };

    let area = 0.5 * (r.powi(2) * (alpha1 + alpha2 - phi1 - phi2) + a.powi(2)*t_a.sin()/(phi1.cos()*phi2.cos()));
    //println!(" AREA: {:.10}", area);
    area
}

fn get_intersect_area(cx: f64, cy: f64, r: f64) -> f64 {
    //в любом случае
    if r > SQRT_2 {
        return 1.0;
    };
    let rcx = 1.0 - cx;
    let rcy = 1.0 - cy;
    let area = 
    // x = 1
        get_circle_line_area(rcx, cy, r)
    // y = 1
        + get_circle_line_area(rcy, rcx, r)
    // x = 0    
        + get_circle_line_area(cx, rcy, r)
    // y = 0    
        + get_circle_line_area(cy, cx, r);
    //println!("cx {:.3} cy {:.3} R {:.3} AREA{:.10}",cx,cy,r,area);
    area
}

#[cfg(test)]
mod test {
    #[warn(unused_imports)]
    use core::panic;

    use super::*;
    use rand::seq::SliceRandom;

    use rand::Rng;

    #[test]
    fn random_dot() {
        let mut rng = rand::thread_rng();
        let max_dots: usize = rng.gen_range(300..=1000);
        let r = rng.gen_range(0.001..=2.0);

        for cd in 1..=max_dots {
            let r = rng.gen_range(0.001..=2.0);
            let a = rng.gen_range(0.001..=0.999);
            let b = rng.gen_range(0.001..=0.999);
            get_intersect_area(a, b, r);
        }
        assert_eq!(0, 0);
    }
}
