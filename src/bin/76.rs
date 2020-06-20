const N: usize = 100;
fn main() {
    let mut d = vec![0u32; N + 5];

    d[0] = 1;
    for i in 1..=N {
        for j in i..=N {
            d[j] += d[j - i];
        }
    }

    println!("{}", d[N] - 1);
}
