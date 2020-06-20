use primal;

fn main() {
    let sieve = primal::Sieve::new(10000);
    for a in sieve.primes_from(1000).take_while(|&x| x <= 9999) {
        for b in sieve.primes_from(a + 1).take_while(|&x| x <= 9999) {
            let c = b + b - a;
            if c <= 9999 && sieve.is_prime(c) {
                // permutation?
                let mut sa: Vec<char> = a.to_string().chars().collect();
                let mut sb: Vec<char> = b.to_string().chars().collect();
                let mut sc: Vec<char> = c.to_string().chars().collect();
                sa.sort();
                sb.sort();
                sc.sort();
                if sa == sb && sb == sc {
                    if a == 1487 && b == 4817 && c == 8147 {
                        continue;
                    }
                    println!("{}{}{}", a, b, c);
                    return;
                }
            }
        }
    }
}
