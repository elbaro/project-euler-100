use primal;

use rayon::prelude::*;

fn main() {
    let n = 28123;
    let sieve = primal::Sieve::new(n);
    let abundants: Vec<usize> = (1..=n)
        .filter(|&x| {
            sieve
                .factor(x)
                .unwrap()
                .iter()
                .map(|&(p, k)| (p.pow(k as u32 + 1) - 1) / (p - 1))
                .product::<usize>()
                - x
                > x
        })
        .collect();

    let ans: usize = (1..n + 1)
        .into_par_iter()
        .filter(|&x| {
            !abundants
                .iter()
                .take_while(|&&first| first < x)
                .any(|first| {
                    let second = x - first;
                    abundants.binary_search(&second).is_ok()
                })
        })
        .sum();
    println!("{}", ans);
}
