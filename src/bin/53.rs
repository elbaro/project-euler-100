#![feature(inclusive_range_syntax)]

fn main() {
    let mut c = vec![vec![0u64; 101]; 101];
    let t = 1_000_000;
    let mut ans = 0;

    c[0][0] = 1;
    for i in 1 ..= 100 {
        c[i][0] = 1;
        for j in 1 ..= 100 {
            c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
            if c[i][j] > t {
                c[i][j] = t + 1;
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
