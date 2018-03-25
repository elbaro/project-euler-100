extern crate ramp;
extern crate rayon;

use ramp::Int;
use rayon::prelude::*;

fn main() {
    #[allow(non_snake_case)]
    let ans = (1..1001).into_par_iter().max_by_key(|&D| {
        let sq = (D as f32).sqrt() as i32;
        if sq*sq==D{ return Int::zero(); }
        let sq = Int::from(sq);
        let D = Int::from(D);

        let mut a = Int::zero();
        let mut b = Int::one();
        let mut c = Int::one();

        let mut h = Int::from(1);
        let mut k = Int::from(0);
        let mut h_ = Int::from(0);
        let mut k_ = Int::from(1);


        let x = loop {
            let whole = (&a + &b * &sq) / &c;
            a -= &c * &whole;

            let na = &a * &c;
            let nb = -&b * &c;
            let nc = &a * &a - &b * &b * &D;

            let g = na.gcd(&nb).gcd(&nc)*nc.sign();
            a = na/&g;
            b = nb/&g;
            c = nc/&g;

            let nh = &whole*&h + &h_;
            let nk = &whole*&k + &k_;
            let g = nh.gcd(&nk);

            h_ = h;
            k_ = k;
            h = nh / &g;
            k = nk / &g;

            if &h*&h - &D*&k*&k == Int::one() {
                break h;
            }
        };

        x
    }).unwrap();
    println!("{}", ans);
}