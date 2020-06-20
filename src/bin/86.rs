// for (a,b,c) where a<=b<=c, the shortest path is (c^2 + (a+b)^2)^0.5.
// (a,b,c) can be generated by m>n>0, k(m^2-n^2, 2mn)
// but brute force is enough

use rayon::prelude::*;

fn count(m: i32) -> i32 {
    (1..=m)
        .into_par_iter()
        .map(|a| {
            let mut cnt = 0;
            for b in a..=m {
                for c in b..=m {
                    let d = c * c + (a + b) * (a + b);
                    let sqrt = (d as f64).sqrt() as i32;
                    if sqrt * sqrt == d {
                        cnt += 1;
                    }
                }
            }
            cnt
        })
        .sum()
}

fn main() {
    let mut left = 1000;
    let mut right = 5000;
    const TARGET: i32 = 1_000_000;
    while left < right {
        let mid = (left + right) / 2;
        let cnt = count(mid);
        if cnt < TARGET {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    println!("{}", right);
}
