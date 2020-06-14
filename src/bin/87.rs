// a <= N^1/4 = 84.xx
// b <= N^1/3 = 368.xx
// c <= N^1/2 = 7071.xx

fn main() {
    let sieve = primal::Sieve::new(7072);
    let aa: Vec<usize> = sieve
        .primes_from(1)
        .take_while(|&p| p <= 84)
        .map(|p| p as usize)
        .collect();
    let bb: Vec<usize> = sieve
        .primes_from(1)
        .take_while(|&p| p <= 368)
        .map(|p| p as usize)
        .collect();

    const M: usize = 50_000_000 - 1;
    let mut found = vec![false; M + 1];

    for a in aa {
        for b in &bb {
            let sum = a * a * a * a + b * b * b;
            for p in sieve.primes_from(1) {
                let p = p as usize;
                let n = sum + p * p;
                if n <= M {
                    found[n] = true;
                } else {
                    break;
                }
            }
        }
    }

    println!("{}", found.iter().map(|&b| b as usize).sum::<usize>());
}
