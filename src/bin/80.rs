use num_bigint::BigUint;

fn main() {
    let t = BigUint::from(10u32).pow(200);
    let mut ans = 0;
    for i in 1..=100i32 {
        let x = (i as f32).powf(0.5) as i32;
        if x * x == i {
            continue;
        }
        // sqrt(i)=x
        // sqrt(i*10^200)=x*10^100
        // x = sqrt(a) = sqrt(i*t)
        let a = BigUint::from(i as u32) * &t;
        let x = a.sqrt();
        let x = x.to_string();

        for ch in (&x[0..100]).bytes() {
            ans += (ch as u32) - '0' as u32;
        }
    }
    println!("{}", ans);
}
