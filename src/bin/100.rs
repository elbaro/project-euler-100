// 2 B/T * (B-1)/(T-1) = 1
// (2T-1)^2 - 2(2B-1)^2 = -1

// x^2 - 2y^2 = -1
// pell's equation with D=2
// generate (xk+1, yk+1) from (xk, yk)
// not every (xk, yk) is a solution

fn main() {
    let x1: i128 = 1;
    let y1: i128 = 1;

    let mut xk: i128 = x1;
    let mut yk: i128 = y1;

    loop {
        let xk1 = x1 * xk + 2 * y1 * yk;
        let yk1 = x1 * yk + y1 * xk;
        xk = xk1; // = 2T-1
        yk = yk1; // = 2B-1
        if (xk * xk - 2 * yk * yk) == -1 && (xk & 1 == 1) && (yk & 1 == 1) {
            let t = (xk + 1) / 2;
            let b = (yk + 1) / 2;
            if t > 1_000_000_000_000 {
                println!("{}", b);
                break;
            }
        }
    }
}
