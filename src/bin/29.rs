use num_bigint::BigUint;
use num_traits;

fn main() {
    let mut s = std::collections::HashSet::new();
    for a in 2..=100 {
        for b in 2..=100 {
            let n = num_traits::pow::pow(BigUint::from(a as usize), b).to_string();
            s.insert(n);
        }
    }
    println!("{}", s.len());
}
