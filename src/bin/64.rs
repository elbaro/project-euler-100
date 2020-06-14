use num::integer::Integer;
use num_bigint::BigInt;
use num_bigint::Sign;
use num_traits::identities::{One, Zero};
use rayon::prelude::*;
use std::collections::HashSet;

fn sign(s: Sign) -> i8 {
    match s {
        Sign::Plus => 1,
        Sign::Minus => -1,
        Sign::NoSign => 0,
    }
}

fn main() {
    let ans = (2..10001)
        .into_par_iter()
        .filter(|&n| {
            let sq = (n as f64).sqrt();
            let m = sq as i32;
            if m * m == n {
                return false;
            }

            let mut s: HashSet<(BigInt, BigInt, BigInt)> = HashSet::new();

            // (a + b*sqrt(n)) / c
            let n = BigInt::from(n);
            let mut a = BigInt::zero();
            let mut b = BigInt::one();
            let mut c = BigInt::one();

            let mut it = -1;
            loop {
                let t = (a.clone(), b.clone(), c.clone());
                if s.contains(&t) {
                    break;
                } else {
                    s.insert(t);
                }

                it += 1;
                // whole = int [ (a + b*sqrt(n)) / c ]
                // whole = int [ (a + [sqrt(b^2+n)]) / c ]
                let sq = (&b * &b + &n).sqrt();
                let whole = (&a + sq) / &c;
                a -= &c * &whole;
                let (na, nb, nc) = (&a * &c, -&b * &c, &a * &a - &b * &b * &n);

                let mut g = na.gcd(&nb).gcd(&nc);
                if sign(nc.sign()) * sign(g.sign()) < 0 {
                    g *= -1;
                }
                a = na / &g;
                b = nb / &g;
                c = nc / &g;
            }

            it % 2 != 0
        })
        .count();
    println!("{}", ans);
}
