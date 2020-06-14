/*
01, 04, 09, 16, 25, 36, 49, 64, 81

1. For each number ab,
(1) a on cube1, b on cube2
(2) a on cube2, b on cube1

2. 09 => 06, 60, 09, 90

3. 64 and 49 are duplicates.
*/

const REQ: [[u32; 4]; 8] = [
    [1, 10, 0, 0],    // 0
    [4, 40, 0, 0],    // 1
    [9, 90, 6, 60],   // 23
    [16, 61, 19, 91], // 45
    [25, 52, 0, 0],   // 6
    [36, 63, 39, 93], // 78
    [49, 94, 46, 64], // 9 10
    [81, 18, 0, 0],   // 11
];

fn gen_cubes(submask: u32) -> Vec<u32> {
    match submask.count_ones() {
        6 => vec![submask],
        5 => {
            let mut v = Vec::new();
            for i in 0..10 {
                if (submask & (1 << i)) == 0 {
                    v.push(submask | (1 << i));
                }
            }
            v
        }
        4 => {
            let mut v = Vec::new();
            for i in 0..10 {
                if submask & (1 << i) == 0 {
                    for j in (i + 1)..10 {
                        if (submask & (1 << j)) == 0 {
                            v.push(submask | (1 << i) | (1 << j));
                        }
                    }
                }
            }
            v
        }
        _ => unreachable!(),
    }
}

fn main() {
    let mut ans = std::collections::HashSet::<u32>::new();
    for i in 0..4096 {
        let reqs = [
            REQ[0][i & 1],
            REQ[1][(i >> 1) & 1],
            REQ[2][(i >> 2) & 3],
            REQ[3][(i >> 4) & 3],
            REQ[4][(i >> 6) & 1],
            REQ[5][(i >> 7) & 3],
            REQ[6][(i >> 9) & 3],
            REQ[7][(i >> 11) & 1],
        ];

        let mut cube1 = 0u32;
        let mut cube2 = 0u32;
        for r in reqs.iter() {
            cube1 |= 1 << (r / 10);
            cube2 |= 1 << (r % 10);
        }
        if cube1.count_ones() > 6 || cube2.count_ones() > 6 {
            continue;
        }
        let c1 = gen_cubes(cube1);
        let c2 = gen_cubes(cube2);
        for low in c1 {
            for high in &c2 {
                let cube = (high << 10) + low;
                ans.insert(cube);
            }
        }
    }

    let mut same = 0;
    for num in &ans {
        let low = num >> 10;
        let high = num & (1024 - 1);
        if low == high {
            same += 1;
        }
    }
    println!("{}", (ans.len() - same) / 2 + same);
}
