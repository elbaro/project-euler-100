fn main() {
    let mut ans = 0;
    let mut mn = 2_000_000;
    for n in 1..2000i64 {
        let a = n*(n+1)/2;
        for m in 1..2000i64 {
            let b = m*(m+1)/2;
            let diff = (a*b - 2_000_000).abs();
            if diff<mn {
                mn = diff;
                ans = n*m;
            }
        }
    }
    println!("{}", ans);
}
