extern crate primal;
extern crate rayon;

use rayon::prelude::*;

fn main() {
    let ans = (1..1000)
        .into_par_iter()
        .max_by_key(|&x| {
            let mut x = x;
            while x % 2 == 0 {
                x /= 2;
            }
            while x % 5 == 0 {
                x /= 5;
            }
            // find (a) s.t. 10^a === 1 (x)
            if x > 1 {
                let mut r = 1;
                for i in 1.. {
                    r = r * 10 % x;
                    if r == 1 {
                        return i;
                    }
                }
            }

            1
        })
        .unwrap();
    println!("{}", ans);
}
