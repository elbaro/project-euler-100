fn main() {
    let mut sum = 1;
    let mut last = 1;
    for n in (3..=1001).step_by(2) {
        for _ in 0..4 {
            last += n - 1;
            sum += last;
        }
    }
    println!("{}", sum);
}
