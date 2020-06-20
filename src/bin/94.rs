use rayon::prelude::*;

fn is_sqrt(num: u64) -> bool {
    let sq = (num as f64).sqrt().round() as u64;
    sq * sq == num
}

fn main() {
    let mut ans = 0u64;

    ans += (2..=333_333_333u64)
        .into_par_iter()
        .map(|i| {
            // (i,i,i-1)
            // i should be odd
            // i=2k+1 (k>=1)
            if i & 1 == 1 {
                // let s = (i+i+i-1) as f32 /2.0; // 1.5i - 0.5  =>  3k + 1
                // let a = sqrt(s*(s-i)*(s-i)*(s-i+1));  // sqrt((3k+1)(k)(k)(k+1))
                let k = i / 2;
                if is_sqrt((3 * k + 1) * (k + 1)) {
                    return 3 * i - 1;
                }
            }
            0u64
        })
        .sum::<u64>();

    ans += (2..=333_333_333u64)
        .into_par_iter()
        .map(|i| {
            if i & 1 == 1 {
                // (i,i,i+1)
                // let s = (i+i+i+1) as f32 /2.0; // 1.5i + 0.5  =>  3k + 2
                // let a = sqrt(s*(s-i)*(s-i)*(s-i-1));  // sqrt((3k+2)(2k+1)(2k+1)(k))
                let k = i / 2;
                if is_sqrt((3 * k + 2) * k) {
                    return 3 * i + 1;
                }
            }
            0u64
        })
        .sum::<u64>();

    println!("{}", ans);
}
