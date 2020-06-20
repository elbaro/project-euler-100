use rayon::prelude::*;

fn main() {
    let fifth: Vec<usize> = (0..10).map(|x: usize| x.pow(5)).collect();

    let ans: usize = (2usize..10_000_000)
        .into_par_iter()
        .filter(|&n| {
            let mut m = n;
            let mut s = 0;
            while m > 0 {
                s += fifth[m % 10];
                m /= 10;
            }
            s == n
        })
        .sum();
    println!("{}", ans);
}
