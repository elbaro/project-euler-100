#![feature(inclusive_range_syntax)]
extern crate rayon;

use rayon::prelude::*;

fn main() {
    let ans = (3..1001).into_par_iter().max_by_key(|&p| {
        let mut cnt = 0;
        for a in 1..p {
            for b in a..p {
                let c = p - a - b;
                if b > c { continue; }
                if a * a + b * b == c * c {
                    cnt += 1;
                }
            }
        }
        cnt
    }).unwrap();
    println!("{}", ans);
}
