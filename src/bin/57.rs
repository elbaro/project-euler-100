use num_bigint::BigUint;
use num_traits::identities::One;
fn main() {
    let mut f = (BigUint::one(), BigUint::one());
    let mut ans = 0;
    for _ in 0..1000 {
        f.0 += &f.1;
        f = (f.1 + &f.0, f.0);
        if f.0.to_string().len() > f.1.to_string().len() {
            ans += 1;
        }
    }
    println!("{:}", ans);
}
