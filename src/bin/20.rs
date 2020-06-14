extern crate num_bigint;
use num_bigint::BigUint;

fn main() {
    let ans: usize = (1..=100)
        .map(|x| BigUint::from(x as usize))
        .fold(BigUint::from(1usize), |p, x| p * x)
        .to_string()
        .chars()
        .map(|ch| (ch as usize) - ('0' as usize))
        .sum();
    println!("{}", ans);
}
