extern crate ramp;
use ramp::Int;
extern crate rayon;
use rayon::prelude::*;

fn main() {
    let ans = (2..100).into_par_iter().map(|a| {
        (2..100).into_par_iter().map(|b:i32| {
            let pow = Int::from(a).pow(b as usize);
            pow.to_str_radix(10, false).chars().map(|c| (c as u8) - '0' as u8).fold(0 as u32, |x,y| x+y as u32)
        }).max().unwrap()
    }).max().unwrap();
    println!("{}", ans);
}