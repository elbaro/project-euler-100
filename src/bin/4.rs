fn main() {
    let mut ans = 0;
    for i in 100..=999 {
        for j in 100..=999 {
            let k = i * j;
            if k < ans {
                continue;
            }

            let s1: String = k.to_string();
            let s2: String = s1.chars().rev().collect();
            if s1 == s2 {
                ans = k;
            }
        }
    }

    println!("{}", ans);
}
