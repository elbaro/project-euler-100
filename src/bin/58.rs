extern crate primal;

fn main() {
    let mut n = 1;
    let mut cnt = 0;
    let sieve = primal::Sieve::new(1_000_000_000);
    for s in 1.. {
        for _ in 0..4 {
            n += 2*s;
            if sieve.is_prime(n) {
                cnt += 1;
            }
        }
        if cnt*10<s*4+1 {
            println!("{}",s*2+1);
            break;
        }
    }
}
