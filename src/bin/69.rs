use ordered_float;

fn main() {
    const N: usize = 1_000_000;
    let mut pi = vec![0usize; N + 1];
    pi[1] = 1;
    for i in 2..N + 1 {
        if pi[i] == 0 {
            let mut j = i * 2;
            while j <= N {
                pi[j] = i;
                j += i;
            }
            pi[i] = i - 1;
        } else {
            let p = pi[i];
            let mut x = i;
            while x % p == 0 {
                x /= p;
            }
            if x == 1 {
                pi[i] = i - i / p;
            } else {
                pi[i] = pi[x] * pi[i / x];
            }
        }
    }

    let ans = (1..N + 1)
        .max_by_key(|&x| ordered_float::OrderedFloat(x as f64 / pi[x] as f64))
        .unwrap();
    println!("{}", ans);
}
