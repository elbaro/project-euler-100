fn main() {
    const MOD: u64 = 1e10 as u64;

    let ans = (1..1001u64)
        .map(|x| (0..x).fold(1u64, |product, _| product * x % MOD))
        .fold(0u64, |x, y| (x + y) % MOD);
    println!("{}", ans);
}
