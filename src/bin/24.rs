fn main() {
    let fact: Vec<usize> = std::iter::once(1)
        .chain((1..10).scan(1, |f, x| {
            *f *= x;
            Some(*f)
        }))
        .collect();
    let mut digits: Vec<usize> = (0..10).collect();

    let mut n = 1_000_000 - 1;
    let mut ans: String = "".to_string();
    for i in 0..10 {
        let index = n / fact[9 - i];
        n -= index * fact[9 - i];
        ans += &digits[index].to_string();
        digits.remove(index);
    }

    println!("{:}", ans);
}
