use rayon::prelude::*;

fn main() {
    let ans = (1..1000)
        .into_par_iter()
        .filter(|i| i % 3 == 0 || i % 5 == 0)
        .count();
    println!("{}", ans);
}
