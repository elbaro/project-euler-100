#![feature(inclusive_range_syntax)]

fn main() {
    let ans = {
        let mut ret = 0;
        'e:for a in 1..=1000 {
            for b in a..=1000 {
                let c = 1000 - a - b;
                let cc = a*a + b*b;
                if c*c == cc {
                    ret = a*b*c;
                    break 'e;
                }
            }
        }
        ret
    };
    println!("{}", ans);
}
