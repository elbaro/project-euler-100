extern crate primal;

use std::collections::HashSet;

fn main() {
    let mut p: HashSet<usize> = HashSet::new();
    let mut ten = 1;

    let ans: usize = primal::Primes::all()
        .filter(|&n| {
            p.insert(n);
            if ten * 10 <= n {
                ten *= 10;
            }
            if n < 10 {
                return false;
            }

            let mut x = n;
            let mut ok = true;

            while x > 0 {
                if !p.contains(&x) {
                    ok = false;
                    break;
                }
                x /= 10;
            }

            if ok {
                x = n;
                let mut t = ten;

                while t > 0 {
                    if !p.contains(&x) {
                        ok = false;
                        break;
                    }
                    x -= x / t * t;
                    t /= 10;
                }
            }
            ok
        })
        .take(11)
        .sum();
    println!("{}", ans);
}
