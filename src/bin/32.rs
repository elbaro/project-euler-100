#[macro_use]
extern crate panic_context;

use permutohedron::Heap;
use std::collections::HashSet;

fn main() {
    let mut products: HashSet<u32> = HashSet::new();
    let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let heap = Heap::new(&mut data);
    for data in heap {
        for i in 1..=9 {
            for j in i..=9 {
                if i + j + j > 9 {
                    continue;
                }

                panic_context!("i {} j {}", i, j);

                // [..i..] [..j..] = [..k..]
                let a: u32 = (&data[..i]).iter().fold(0u32, |x, y| x * 10 + y);
                let b = (&data[i..i + j]).iter().fold(0u32, |x, y| x * 10 + y);
                let c = (&data[i + j..]).iter().fold(0u32, |x, y| x * 10 + y);

                if a * b == c {
                    products.insert(c);
                }
            }
        }
    }
    let ans: u32 = products.iter().sum();
    println!("{}", ans);
}
