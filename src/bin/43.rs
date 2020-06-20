

static P: [usize; 11] = [0, 1, 1, 1, 2, 3, 5, 7, 11, 13, 17];

fn f(num: usize, level: usize, used: u32) -> usize {
    let mut sum: usize = 0;
    for d in 0..10 {
        let new = num * 10 + d;
        if new % 1000 % P[level] == 0 && (used & (1u32 << d)) == 0 {
            if level == 10 {
                sum += new;
            } else {
                sum += f(num * 10 + d, level + 1, used | (1u32 << d));
            }
        }
    }
    sum
}

fn main() {
    println!("{}", f(0, 1, 0));
}
