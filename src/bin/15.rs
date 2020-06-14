extern crate num_bigint;
use num_bigint::BigUint;

// 40C20 = 40*39*..21 / 20!
fn main() {
    let mut p = BigUint::from(1u32);
    for i in 21..=40 {
        p *= BigUint::from(i as u32);
    }
    for i in 1..=20 {
        p /= BigUint::from(i as u32);
    }

    println!("{}", p.to_string());
}
