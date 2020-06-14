/**
 * N = a1 + a2 + ... + ak = a1 × a2 × ... × ak.
 *   = 1 + 1 + .. a(ones+1) + .. ak = 1 * 1 * .. a(ones+1) * .. ak
 *   = ones + rest_sum = rest_prod
 *   => ones = rest_prod - rest_sum
 * 0 <= rest_prod - rest_sum <= k
 */

struct Solver {
    mins: Vec<u64>,
}

impl Solver {
    fn recurse(&mut self, prod: u64, sum: u64, cnt: u64, last_num: u64) {
        let k = (prod - sum + cnt) as usize;
        if k > 12000 {
            return;
        }
        // println!("{} {} {} {}", prod, sum, cnt, last_num);
        if prod < self.mins[k] {
            self.mins[k] = prod;
        }
        // first number > 1 cannot be > 12000
        // because second number is >= 12000
        // we already have 12000*12000 - 24000

        // last number cannot be > 12000
        // because (num)*12000 - (num+12000) = 11999 num - 12000 ~= 120000
        for i in last_num..12000 {
            let kk = prod * i - (sum + i) + (cnt + 1);
            if kk > 12000 {
                break;
            }
            self.recurse(prod * i, sum + i, cnt + 1, i);
        }
    }
}

fn main() {
    let mut solver = Solver {
        mins: vec![999999; 12001],
    };
    solver.recurse(1, 0, 0, 2);
    solver.mins[0] = 0;
    solver.mins[1] = 0;
    solver.mins.sort();
    solver.mins.dedup();

    let ans: u64 = solver.mins.iter().sum();
    println!("{}", ans);
}
