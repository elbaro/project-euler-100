extern crate primal;

fn main() {
    let sieve = primal::Sieve::new(1_000_000 - 1);
    let mut digits: [u8; 10] = unsafe { std::mem::uninitialized() };
    let tens: [u32; 10] = [
        1, 10, 1e2 as u32, 1e3 as u32, 1e4 as u32, 1e5 as u32, 1e6 as u32, 1e7 as u32, 1e8 as u32,
        1e9 as u32,
    ];
    'f: for p in sieve.primes_from(100) {
        let mut n = 0;
        let mut x = p as u32;
        while x > 0 {
            digits[n] = (x % 10) as u8;
            n += 1;
            x /= 10;
        }

        // num of digits to change: 3, 6, 9 ..
        // otherwise, one of 8 primes is a multiple of 3.
        for i in 0..n {
            if digits[i] > 2 {
                continue;
            } // start digit: 0 or 1 or 2
            for j in i + 1..n {
                if digits[i] != digits[j] {
                    continue;
                }
                for k in j + 1..n {
                    if digits[j] == digits[k] {
                        let mut x = p as u32;
                        let mut cnt = 10 - digits[i];
                        for _d in digits[i] + 1..10 {
                            x += tens[i] + tens[j] + tens[k];
                            if !sieve.is_prime(x as usize) {
                                cnt -= 1;
                                if cnt < 8 {
                                    break;
                                }
                            }
                        }
                        if cnt >= 8 {
                            println!("{}", p);
                            break 'f;
                        }
                    }
                }
            }
        }
    }
}
