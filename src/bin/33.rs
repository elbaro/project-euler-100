#![feature(inclusive_range_syntax)]
extern crate num;
extern crate panic_context;

use num::Integer;

fn main() {
    let mut x:u32 = 1;
    let mut y:u32 = 1;
    for a in 10..100 {
        for b in a+1..100 {
            if a%10==b/10 && a*(b%10) == b*(a/10) {
                x *= a;
                y *= b;
//                println!("{} / {}", a, b);
            }
        }
    }
    let g = x.gcd(&y);
    let ans = y/g;
    println!("{}", ans);
}
