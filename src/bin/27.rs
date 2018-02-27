#![feature(inclusive_range_syntax)]
#[macro_use] extern crate itertools;
extern crate primal;

fn main() {
    let bs = primal::Primes::all().take_while(|&x| x<=1_000);
    let ps:Vec<isize> = primal::Primes::all().take_while(|&x| x<=1_000_000).map(|x| x as isize).collect();

    let (b,a) = iproduct!(bs, -1000isize..=1000).max_by_key(
        |&(b,a)| {
            (0..).map(|n| n*n + a*n + b as isize).take_while(|&x| ps.binary_search(&x).is_ok()).count()
        }
    ).unwrap();
    println!("{}", a*b as isize);
}
