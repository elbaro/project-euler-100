fn main() {
    let (mut a, mut b) = (1, 2);
    let mx = 4_000_000;

    let mut ans = 0;
    while a < mx {
        if a % 2 == 0 {
            ans += a;
        }
        let c = a + b;
        a = b;
        b = c;
    }

    println!("{}", ans);
}
