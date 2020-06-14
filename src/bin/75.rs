extern crate num;
fn main() {
    const L: usize = 1_500_000;
    let mut visit = vec![0; L + 1];
    let mut ans = 0;
    for n in (1..=L).step_by(2) {
        for m in (n + 2..=L).step_by(2) {
            let l = m * (m + n);
            if l > L {
                break;
            }

            if num::integer::gcd(n, m) == 1 {
                let mut ll = l;
                while ll <= L {
                    visit[ll] += 1;
                    ll += l;
                }
            }
        }
    }

    for i in 3..=L {
        if visit[i] == 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
