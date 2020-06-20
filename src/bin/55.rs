use num_bigint::BigInt;


fn main() {
    let ans = (1..=10_000)
        .filter(|&start| {
            let mut x = BigInt::from(start);
            for i in 0..51 {
                let xs = x.to_string();
                let ys = xs.chars().rev().collect::<String>();

                if i > 0 && xs == ys {
                    return false;
                }
                let y = ys.parse::<BigInt>().unwrap();
                x += y;
            }

            true
        })
        .count();
    println!("{}", ans);
}
