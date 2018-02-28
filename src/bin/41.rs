#![feature(inclusive_range_syntax)]
extern crate primal;
extern crate permutohedron;

use permutohedron::Heap;

fn solve(n:u32) -> Option<u32> {
    let mut data:Vec<u32> = (1..n+1).collect();
    let heap = Heap::new(&mut data);
    let mut ans:Option<u32> = None;
    for data in heap {
        let mut num = 0;
        for digit in data {
            num = num*10 + digit;
        }


        if (ans.is_none() || num>ans.unwrap()) && primal::is_prime(num as u64) {
            ans = Some(num);
        }
    }
    ans
}

fn main() {
    for n in (1..10).rev() {
        if let Some(ans) = solve(n) {
            println!("{}", ans);
            break;
        }
    }
}
