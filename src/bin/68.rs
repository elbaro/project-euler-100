/*
      a
           b      d
     i          c
j       g-----e-----f
           h
*/
fn main() {
    let mut ans = "".to_string();
    for a in 1..7 {
        for b in 1..10 {
            if a == b {
                continue;
            }
            for c in 1..10 {
                if a == c || b == c {
                    continue;
                }
                let sum = a + b + c;
                let flag = 1 << a | 1 << b | 1 << c;

                for d in a + 1..11 {
                    if (flag & (1 << d)) != 0 {
                        continue;
                    }
                    let flag = flag ^ (1 << d);
                    let e = sum - d - c;
                    if e < 1 || e > 9 || (flag & (1 << e)) != 0 {
                        continue;
                    }
                    let flag = flag ^ (1 << e);

                    for f in a + 1..11 {
                        if (flag & (1 << f)) != 0 {
                            continue;
                        }
                        let flag = flag ^ (1 << f);
                        let g = sum - e - f;
                        if g < 1 || g > 9 || (flag & (1 << g)) != 0 {
                            continue;
                        }
                        let flag = flag ^ (1 << g);

                        for h in a + 1..11 {
                            if (flag & (1 << h)) != 0 {
                                continue;
                            }
                            let flag = flag ^ (1 << h);
                            let i = sum - h - g;
                            let j = sum - i - b;
                            if i > 0 && j > 0 && (flag ^ (1 << i) ^ (1 << j) == 0b11111111110) {
                                let s = a.to_string()
                                    + &b.to_string()
                                    + &c.to_string()
                                    + &d.to_string()
                                    + &c.to_string()
                                    + &e.to_string()
                                    + &f.to_string()
                                    + &e.to_string()
                                    + &g.to_string()
                                    + &h.to_string()
                                    + &g.to_string()
                                    + &i.to_string()
                                    + &j.to_string()
                                    + &i.to_string()
                                    + &b.to_string();
                                if s > ans {
                                    ans = s;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
