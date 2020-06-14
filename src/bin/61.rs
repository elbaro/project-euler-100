fn dfs(p: &Vec<Vec<bool>>, nums: &[usize], flag: u32) -> bool {
    if nums.len() == 0 {
        true
    } else {
        for i in 0..6 {
            if p[i][nums[0]] && flag & (1 << i) > 0 {
                if dfs(p, &nums[1..], flag ^ (1 << i)) {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    let mut p = vec![vec![false; 10_000]; 7];

    let fs: [Box<Fn(usize) -> usize>; 6] = [
        Box::new(|n| n * (n + 1) / 2),
        Box::new(|n| n * n),
        Box::new(|n| n * (3 * n - 1) / 2),
        Box::new(|n| n * (2 * n - 1)),
        Box::new(|n| n * (5 * n - 3) / 2),
        Box::new(|n| n * (3 * n - 2)),
    ];
    for (i, f) in fs.into_iter().enumerate() {
        (1usize..)
            .map(|n| f(n))
            .take_while(|&x| x < 10_000)
            .for_each(|x| {
                p[i][x] = true;
                p[6][x] = true;
            });
    }

    // aabb bbcc ccdd ddee eeff ffaa
    for a in 10..100 {
        for b in a..100 {
            if !p[6][a * 100 + b] {
                continue;
            }
            for c in a..100 {
                if !p[6][b * 100 + c] {
                    continue;
                }
                for d in a..100 {
                    if !p[6][c * 100 + d] {
                        continue;
                    }
                    for e in a..100 {
                        if !p[6][d * 100 + e] {
                            continue;
                        }
                        'entry: for f in a..100 {
                            if p[6][e * 100 + f] && p[6][f * 100 + a] {
                                let nums = [
                                    a * 100 + b,
                                    b * 100 + c,
                                    c * 100 + d,
                                    d * 100 + e,
                                    e * 100 + f,
                                    f * 100 + a,
                                ];
                                if dfs(&p, &nums, 0b111111) {
                                    println!("{}", nums.iter().sum::<usize>());
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
