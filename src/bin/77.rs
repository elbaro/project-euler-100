extern crate primal;

const N:usize = 10000;
fn main() {
    let sieve = primal::Sieve::new(N);
    let mut d=vec![0u32;N+5];
    let mut ans = 0;
    d[0]=1;
    for p in sieve.primes_from(2).take_while(|x| *x<=N) {
        for i in p..=N {
            d[i] += d[i-p];
        }
    }
    for i in 1..=N {
        if d[i]>5000 {
            ans = i;
            break;
        }
    }
    println!("{}", ans);
}
