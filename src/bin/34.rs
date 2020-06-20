use rayon::prelude::*;

fn main() {
    let f = {
        let mut f = [0; 10];
        f[0] = 1;
        for i in 1..10 {
            f[i] = f[i - 1] * i;
        }
        f
    };

    let ans: u64 = (10..100000000)
        .into_par_iter()
        .filter(|&n: &i32| {
            let mut x = n;
            let mut s: usize = 0;
            while x > 0 {
                s += f[(x % 10) as usize];
                x /= 10;
            }
            n == s as i32
        })
        .map(|x| x as u64)
        .sum();
    println!("{}", ans);
}
