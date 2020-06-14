fn main() {
    let mut ans = "".to_string();
    for x in 1..10_000 {
        let mut flag: usize = 0;
        let mut s = "".to_string();

        'l: for n in 1usize.. {
            let num = x * n;
            s += &num.to_string();

            let mut k = num;
            while k > 0 {
                if flag & (1 << (k % 10)) > 0 {
                    break 'l;
                }
                flag |= 1 << (k % 10);
                k /= 10;
            }

            if flag == 0b1111111110 {
                // found pandigital
                if s > ans {
                    ans = s.clone();
                }
            }
        }
    }

    // x*1..x*2..x*3..x*n
    // x<=4 digits
    println!("{}", ans);
}
