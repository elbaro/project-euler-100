/**
28433×2^7830457+1
**/

const COEFF: u128 = 28433;
const POW: u128 = 7830457;
const TEN_ZEROS: u128 = 1_00000_00000;

fn main() {
    let mut p = 1;
    let mut pow = 2u128;
    let mut num = 1u128;
    loop {
        if p > POW {
            break;
        }
        if (POW & p) != 0 {
            // at this point, pow = 2^p
            num = (num * pow) % TEN_ZEROS;
        }
        p <<= 1;
        pow = (pow * pow) % TEN_ZEROS;
    }
    num = (COEFF * num) % TEN_ZEROS;
    num = (num + 1) % TEN_ZEROS;
    println!("{}", num);
}
