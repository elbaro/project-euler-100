
use primal;

fn main() {
    let mut ans = 0;
    let mut n = 0;
    let sieve = primal::Sieve::new(10000);
    for i in 1.. {
        n += i;
        let f = sieve.factor(n).unwrap();
        let cnt: u32 = f.iter().map(|&(_p, e)| (e + 1) as u32).product();
        if cnt > 500 {
            ans = n;
            break;
        }
    }

    println!("{}", ans);
}
