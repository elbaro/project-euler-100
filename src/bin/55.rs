#![feature(inclusive_range_syntax)]
extern crate ramp;

fn main() {
    let ans = (1..=10_000).filter(|&start| {
        let mut x = ramp::Int::from(start);
        for i in 0..51 {
            let mut xs = x.to_str_radix(10,false);
            let ys = xs.chars().rev().collect::<String>();

            if i>0 && xs==ys {
//                println!("{} -> {}", start, xs);
                return false;
            }
            let y = ramp::Int::from_str_radix(&ys, 10).unwrap();
            x += y;
        }

        true
    }).count();
    println!("{}", ans);
}
