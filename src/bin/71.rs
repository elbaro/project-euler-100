fn main() {
    // 2/5 + .. < 3/7
    const times: u32 = (1_000_000 - 5) / 7;
    const a: u32 = 2 + 3 * times;
    println!("{}", a);
}
