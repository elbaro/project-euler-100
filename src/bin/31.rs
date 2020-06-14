fn main() {
    let coins = vec![1, 2, 5, 10, 20, 50, 100, 200usize];
    const N: usize = 200;
    let mut d = [0; N + 1];

    d[0] = 1;
    for coin in coins {
        for x in coin..=N {
            d[x] += d[x - coin];
        }
    }

    println!("{}", d[N]);
}
