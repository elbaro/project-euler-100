fn main() {
    let ans = (1..1_000_000)
        .filter(|&x| {
            let base10 = x.to_string();
            if base10.chars().eq(base10.chars().rev()) {
                let base2 = format!("{:b}", x);
                base2.chars().eq(base2.chars().rev())
            } else {
                false
            }
        })
        .sum::<i32>();
    println!("{}", ans);
}
