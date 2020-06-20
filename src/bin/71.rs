fn main() {
    // 2/5 + .. < 3/7
    const TIMES: u32 = (1_000_000 - 5) / 7;
    const ANS: u32 = 2 + 3 * TIMES;
    println!("{}", ANS);
}
