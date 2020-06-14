extern crate primal;

fn main() {
    let mut ans: u64 = 1;
    for i in (1..=20).rev() {
        if let Some(_) = primal::as_prime_power(i) {
            if ans % i != 0 {
                ans *= i;
            }
        }
    }

    println!("{}", ans);
}
