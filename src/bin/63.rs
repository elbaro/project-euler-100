// 10^(n-1) <= x^n < 10^n
// -> n <= 21
// 10^((n-1)/n) <= x < 10
extern crate ramp;

fn main() {
    let mut ans = 0;
    for n in 1..22 {
        for x in 1..10 {
            let k = ramp::Int::from(x).pow(n);
            let len = k.to_string().len();
            if len==n {
                ans += 1;
            }
        }

    }
    println!("{}", ans);
}