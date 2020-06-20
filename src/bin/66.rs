use num::integer::Integer;
use num_bigint::BigInt;
use num_bigint::Sign;
use num_traits::identities::{One, Zero};
use rayon::prelude::*;

fn main() {
    #[allow(non_snake_case)]
    let ans = (1..1001)
        .into_par_iter()
        .max_by_key(|&D| {
            let sq = (D as f32).sqrt() as i32;
            if sq * sq == D {
                return BigInt::zero();
            }
            let sq = BigInt::from(sq);
            let D = BigInt::from(D);

            let mut a = BigInt::zero();
            let mut b = BigInt::one();
            let mut c = BigInt::one();

            let mut h = BigInt::from(1);
            let mut k = BigInt::from(0);
            let mut h_ = BigInt::from(0);
            let mut k_ = BigInt::from(1);

            let x = loop {
                let whole = (&a + &b * &sq) / &c;
                a -= &c * &whole;

                let na = &a * &c;
                let nb = -&b * &c;
                let nc = &a * &a - &b * &b * &D;

                let g = na.gcd(&nb).gcd(&nc)
                    * match nc.sign() {
                        Sign::Plus => 1,
                        Sign::Minus => -1,
                        Sign::NoSign => 0,
                    };
                a = na / &g;
                b = nb / &g;
                c = nc / &g;

                let nh = &whole * &h + &h_;
                let nk = &whole * &k + &k_;
                let g = nh.gcd(&nk);

                h_ = h;
                k_ = k;
                h = nh / &g;
                k = nk / &g;

                if &h * &h - &D * &k * &k == BigInt::one() {
                    break h;
                }
            };

            x
        })
        .unwrap();
    println!("{}", ans);
}
