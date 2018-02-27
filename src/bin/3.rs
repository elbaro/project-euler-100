use std::cell::Cell;

fn main() {
    let n = Cell::new(600851475143u64);
    let mut ans = 0;

    (2..).take_while(|&x| (x as u64) * (x as u64) <= n.get()).for_each(|x| {
        let mut _n = n.get();
        if _n % x == 0 {
            if x > ans { ans = x; }
            while _n % x == 0 { _n /= x; }
            n.set(_n);
        }
    });

    let _n = n.get();
    if _n > 1 {
        if _n > ans {
            ans = _n;
        }
    }

    println!("{}", ans);
}
