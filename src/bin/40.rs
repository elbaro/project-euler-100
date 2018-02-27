#![feature(inclusive_range_syntax)]
extern crate rayon;

fn f(mut n:i32) -> i32 {
    let mut cnt = 9;
    let mut last = 0;
    for digit in 1.. {
        if n<=digit*cnt {
            let mut num = last+(n-1)/digit+1;
            let pos = digit - (n - (n-1)/digit*digit); // 1~
            for _ in 0..pos {
                num /= 10;
            }
            return num%10;
        } else {
            n -= digit*cnt;
        }
        last += cnt;
        cnt *= 10;
    }
    0
}

fn main() {
    let ans:i32 = (0..=6).map(|x| f(10i32.pow(x)))
        .product();
    println!("{}", ans);
}
