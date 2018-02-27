extern crate primal;

fn main() {
    let mut ans = 0;
    let mut digits = 1;
    let mut ten = 1;
    let sieve = primal::Sieve::new(1_000_000);

    for x in sieve.primes_from(1) {
        while ten * 10 <= x {
            ten *= 10;
            digits += 1;
        }
        let mut x = x;
        let mut ok = true;
        for _ in 1..digits {
            let first = x / ten;
            x = x % ten * 10 + first;
            if !sieve.is_prime(x) {
                ok = false;
                break;
            }
        }
        if ok { ans += 1; }
    }
    println!("{}", ans);
}
