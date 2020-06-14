// 10^(n-1) <= x^n < 10^n
// -> n <= 21
// 10^((n-1)/n) <= x < 10
use num_bigint::BigUint;

fn main() {
    let mut ans = 0;
    for n in 1..22 {
        for x in 1..10u8 {
            let k = BigUint::from(x).pow(n);
            let len = k.to_string().len();
            if len == n as usize {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
