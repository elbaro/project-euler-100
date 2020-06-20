

use num_bigint::BigUint;

fn main() {
    let mut a = BigUint::from(1usize);
    let mut b = BigUint::from(1usize);
    let mut i = 2;

    loop {
        i += 1;
        let c = a + &b;
        if c.to_str_radix(10).len() == 1000 {
            break;
        }
        a = b;
        b = c;
    }

    println!("{:}", i);
}
