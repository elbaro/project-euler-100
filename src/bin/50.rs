extern crate primal;

fn main() {
    let sieve = primal::Sieve::new(1_000_000);
    let mut longest = 0;
    let mut ans = 0;

    for start in sieve.primes_from(1) {
        let mut sum = 0usize;
        let mut cnt = 0;
        for p in sieve.primes_from(start) {
            sum += p;
            if sum >= 1_000_000 {
                break;
            }
            cnt += 1;
            if cnt > longest && sieve.is_prime(sum) {
                longest = cnt;
                ans = sum;
            }
        }
    }
    println!("{}", ans);
}
