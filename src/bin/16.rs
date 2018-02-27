#![feature(inclusive_range_syntax)]
extern crate num_bigint;
use num_bigint::BigUint;

fn main() {
    let mut p = BigUint::from(1u32);
    for i in 0..1000 {
        p *= BigUint::from(2u32);
    }
    let ans:usize = p.to_string().chars().map(|ch| (ch as usize) - ('0' as usize)).sum();

    println!("{}", ans);
}
