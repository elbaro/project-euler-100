#![feature(inclusive_range_syntax)]

fn main() {
    let s1:u32 = (1..=100).map(|x| x*x).sum();
    let s2:u32 = (1..=100).sum();
    println!("{}", s2*s2 - s1);
}
