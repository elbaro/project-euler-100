#![feature(inclusive_range_syntax)]
extern crate primal;

fn main() {
    let n = 10000;
    let sieve = primal::Sieve::new(n);
    let d:Vec<usize> = std::iter::once(0).chain((1..n).map(
        |x| {
            sieve.factor(x).unwrap().iter().map(|&(p,k)| (p.pow(k as u32+1)-1)/(p-1)).product::<usize>() - x
        }
    )).collect();
    let ans:usize = (1..n).filter(|&x| x!=d[x] && d[x]<n && x==d[d[x]]).sum();
    println!("{}", ans);
}