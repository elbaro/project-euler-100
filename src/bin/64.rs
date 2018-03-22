extern crate ramp;
extern crate rayon;

use std::collections::HashSet;
use ramp::Int;
use rayon::prelude::*;

fn main() {
    let ans = (2..10001).into_par_iter().filter(|&n| {
        let sq = (n as f64).sqrt();
        let m = sq as i32;
        if m * m == n { return false; }

        let mut s:HashSet<(Int,Int,Int)> = HashSet::new();

        // (a + b*sqrt(n)) / c
        let n = Int::from(n);
        let mut a = Int::zero();
        let mut b = Int::one();
        let mut c = Int::one();

        let mut it = -1;
        loop {
            let t = (a.clone(),b.clone(),c.clone());
            if s.contains(&t) {
                break;
            } else {
                s.insert(t);
            }

            it += 1;
            // whole = int [ (a + b*sqrt(n)) / c ]
            // whole = int [ (a + [sqrt(b^2+n)]) / c ]
            let (sq, _) = (&b * &b + &n).sqrt_rem().unwrap();
            let whole = (&a + sq) / &c;
            a -= &c * &whole;
            let (na, nb, nc) = (&a * &c, -&b * &c, &a * &a - &b * &b * &n);


            let mut g = na.gcd(&nb).gcd(&nc);
            if nc.sign()*g.sign() < 0 { g.negate(); }
            a = na / &g;
            b = nb / &g;
            c = nc / &g;
        }

        it % 2 != 0
    }).count();
    println!("{}", ans);
}