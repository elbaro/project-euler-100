extern crate ordered_float;
extern crate rayon;
use rayon::prelude::*;

fn main() {
    const N: usize = 10_000_000 - 1;
    let mut pi = vec![0usize; N + 1];
    pi[1] = 1;
    for i in 2..N + 1 {
        if pi[i] == 0 {
            let mut j = i * 2;
            while j <= N {
                pi[j] = i;
                j += i;
            }
            pi[i] = i - 1;
        } else {
            let p = pi[i];
            let mut x = i;
            while x % p == 0 {
                x /= p;
            }
            if x == 1 {
                pi[i] = i - i / p;
            } else {
                pi[i] = pi[x] * pi[i / x];
            }
        }
    }

    let ans = (2..N + 1)
        .into_par_iter()
        .filter(|&n| {
            let mut a = n.to_string().into_bytes();
            let mut b = pi[n].to_string().into_bytes();
            a.sort();
            b.sort();
            a == b
        })
        .min_by_key(|&x| ordered_float::OrderedFloat(x as f64 / pi[x] as f64))
        .unwrap();
    println!("{}", ans);
}
