extern crate rand;
extern crate rayon;
// #[macro_use]
// extern crate lazy_static;

use rand::distributions::Uniform;
use rand::{Rng, ThreadRng};
use rayon::prelude::*;

// lazy_static! {
//     static ref dice_dist:Uniform<u8> = Uniform::new(1,5);
// }

const GO: u8 = 0;
const JAIL: u8 = 10;

fn roll(rng: &mut ThreadRng) -> (u8, bool) {
    // let _:() = dice_dist;
    let dice_dist: Uniform<u8> = Uniform::new(1, 5);
    let r1 = rng.sample(&dice_dist);
    let r2 = rng.sample(&dice_dist);
    return (r1 + r2, r1 == r2);
}

fn main() {
    let trials = 5_000_000;
    let card_dist: &Uniform<u8> = &Uniform::new(0, 16);
    let mut rng = rand::thread_rng();
    let mut p = GO;
    let mut doubles = 0;

    let mut c = vec![0; 40];
    let mut total = 0;

    for _ in 0..trials {
        let (r, is_double) = roll(&mut rng);

        if is_double {
            doubles += 1;
            if doubles == 3 {
                p = JAIL;
            } else {
                p = (p + r) % 40;
            }
        } else {
            doubles = 0;
            p = (p + r) % 40;
        }

        loop {
            if p == 30 {
                // G2J
                p = JAIL;
            } else if p == 2 || p == 17 || p == 23 {
                // CC
                let card = rng.sample(card_dist);
                if card == 0 {
                    p = GO;
                } else if card == 1 {
                    p = JAIL;
                } else {
                    break;
                }
            } else if p == 7 || p == 22 || p == 36 {
                // CH
                let card = rng.sample(card_dist);
                if card == 0 {
                    p = GO;
                } else if card == 1 {
                    p = JAIL;
                } else if card == 2 {
                    p = 11;
                } else if card == 3 {
                    p = 24;
                } else if card == 4 {
                    p = 39;
                } else if card == 5 {
                    p = 5;
                } else if card == 6 || card == 7 {
                    // 5 or 15 or 25 or 35
                    p = (p + 1) % 40;
                    while !(p != 0 && p % 5 == 0) {
                        p = (p + 1) % 40;
                    }
                } else if card == 8 {
                    // 12, 28
                    p = (p + 1) % 40;
                    while p != 12 && p != 28 {
                        p = (p + 1) % 40;
                    }
                } else if card == 9 {
                    p = (p + 37) % 40;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        c[p as usize] += 1;
        total += 1;
    }

    let c: Vec<f64> = c.iter().map(|&x| (x as f64) / (total as f64)).collect();
    let mut idx: Vec<usize> = (0..40).collect();
    idx.sort_by(|&a, &b| c[a as usize].partial_cmp(&c[b as usize]).unwrap().reverse());

    // println!("{} {}", idx[0], c[idx[0]]);
    // println!("{} {}", idx[1], c[idx[1]]);
    // println!("{} {}", idx[2], c[idx[2]]);
    println!("{:02}{:02}{:02}", idx[0], idx[1], idx[2]);
}
