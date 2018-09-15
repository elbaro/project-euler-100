extern crate ramp;

use ramp::Int;

fn main() {
    let t = Int::from(10).pow(200);
    let mut ans = 0;
    for i in 1..=100 {
        let x = (i as f32).powf(0.5) as i32;
        if x*x == i {
            continue;
        }
        // sqrt(i)=x
        // sqrt(i*10^200)=x*10^100
        // x = sqrt(a) = sqrt(i*t)
        let a = Int::from(i)*&t;
        let (x,_rem) = a.sqrt_rem().unwrap();
        let x = x.to_str_radix(10, false);

        for ch in (&x[0..100]).bytes() {
            ans += (ch as u32) - '0' as u32;
        }
    }
    println!("{}", ans);
}
