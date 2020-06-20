const N: i32 = 50;

fn main() {
    let mut ans = 0u64;
    for x1 in 0..=N {
        for y1 in 0..=N {
            if x1 == 0 && y1 == 0 {
                continue;
            }
            for x2 in 0..=N {
                for y2 in 0..=N {
                    if x2 == 0 && y2 == 0 {
                        continue;
                    }

                    let dx = x2 - x1;
                    let dy = y2 - y1;
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    if y1 == 0 && x2 == 0 {
                        ans += 1;
                    }
                    // x1:y1 == -dy:dx ?
                    // -dy*y1 == dx*x1 ?
                    else if -dy * y1 == dx * x1 {
                        ans += 1;
                        // println!("{} {} | {} {} ",x1, y1, x2, y2)
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
