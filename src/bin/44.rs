#![feature(inclusive_range_syntax, io)]

fn nth_penta(n : usize) -> usize {
    n*(3*n-1)/2
}

fn is_penta(x : usize) -> bool {
    // 3(n-1)^2 < 2x = n(3n-1) < 3n^2
    let n = ((x*2/3) as f32).sqrt() as usize + 1;
    x==nth_penta(n)
}

fn main() {
    for n_delta in 1.. {
        let delta2 = nth_penta(n_delta)*2;
        let mut ans = false;

        for factor in 1.. {
            if 3*factor*factor > delta2 {
                break;
            }

            if delta2%factor == 0 {
                // b-a = factor
                // 3b+3a-1 = delta2/factor
                // 6a-1 = delta2/factor - 3*factor

                let six_a = delta2/factor - 3*factor + 1;
                if six_a % 6 > 0 { continue; }
                let a = six_a/6;
                let b = factor+a;
                if is_penta(nth_penta(a) + nth_penta(b)) {
                    ans = true;
                    break;
                }
            }
        }

        if ans {
            println!("{}", delta2/2);
            break;
        }
    }
}
