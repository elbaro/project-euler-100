extern crate primal;

fn main() {
    let sieve = primal::Sieve::new(1_000_000_000);
    let primes: Vec<u32> = sieve
        .primes_from(1)
        .map(|p| p as u32)
        .take_while(|&p| p <= 10_000)
        .collect();
    let tens: Vec<u32> = primes
        .iter()
        .map(|&p| {
            let mut x = 1;
            while x <= p {
                x *= 10;
            }
            x
        })
        .collect();
    let n = primes.len();
    let mut a: Vec<Vec<bool>> = vec![vec![false; n + 1]; n + 1];
    for i in 0..n {
        for j in i + 1..n {
            if sieve.is_prime((tens[i] * primes[j] + primes[i]) as usize)
                && sieve.is_prime((tens[j] * primes[i] + primes[j]) as usize)
            {
                a[i][j] = true;
                a[j][i] = true;
            }
        }
    }
    let mut ans = std::u32::MAX;
    for i in 0..n {
        for j in i + 1..n {
            if !a[i][j] {
                continue;
            }
            for k in j + 1..n {
                if !a[i][k] || !a[j][k] {
                    continue;
                }
                for l in k + 1..n {
                    if !a[i][l] || !a[j][l] || !a[k][l] {
                        continue;
                    }
                    for m in l + 1..n {
                        if a[i][m] && a[j][m] && a[k][m] && a[l][m] {
                            let sum = primes[i] + primes[j] + primes[k] + primes[l] + primes[m];
                            if sum < ans {
                                ans = sum;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
