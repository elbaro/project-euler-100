// Grid 01
// 003020600
// 900305001
// 001806400
// 008102900
// 700000008
// 006708200
// 002609500
// 800203009
// 005010300
// Grid 02
// 200080300
// 060070084
// 030500209
// 000105408
// 000000000
// 402706000
// 301007040
// 720040060
// 004010003

fn is_valid(a: &Vec<Vec<u8>>) -> bool {
    // 2 + 4 + .. 512 = (1+2+..512) - 1 = 1023 - 1 = 1022
    for i in 0..9 {
        let mut row = 0;
        let mut col = 0;
        for j in 0..9 {
            if a[i][j] != 0 {
                if (row & (1 << a[i][j])) != 0 {
                    return false;
                }
                row ^= 1 << a[i][j];
            }
            if a[j][i] != 0 {
                if (col & (1 << a[j][i])) != 0 {
                    return false;
                }
                col ^= 1 << a[j][i];
            }
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            let ii = i * 3;
            let jj = j * 3;
            let mut mask = 0;
            for k in ii..ii + 3 {
                for l in jj..jj + 3 {
                    if a[k][l] == 0 {
                        continue;
                    }
                    if (mask & (1 << a[k][l])) != 0 {
                        return false;
                    }
                    mask ^= 1 << a[k][l];
                }
            }
        }
    }
    true
}

fn solve(a: &mut Vec<Vec<u8>>, r: usize, c: usize) -> bool {
    if r == 9 {
        return true;
    }
    if c == 9 {
        return solve(a, r + 1, 0);
    }
    if a[r][c] != 0 {
        return solve(a, r, c + 1);
    }
    for digit in 1..10 {
        a[r][c] = digit;

        if is_valid(a) {
            if solve(a, r, c + 1) {
                return true;
            }
        }
    }

    a[r][c] = 0;

    false
}

fn main() {
    let txt = reqwest::blocking::get("https://projecteuler.net/project/resources/p096_sudoku.txt")
        .unwrap()
        .text()
        .unwrap();

    let lines = txt.lines().collect::<Vec<_>>();
    let mut ans = 0u32;
    for test_case in lines.chunks_exact(10) {
        let mut a: Vec<Vec<u8>> = test_case[1..]
            .iter()
            .map(|s| s.bytes().map(|b| (b - b'0') as u8).collect())
            .collect::<Vec<_>>();
        if !solve(&mut a, 0, 0) {
            unreachable!();
        }
        ans += a[0][0] as u32 * 100 + a[0][1] as u32 * 10 + a[0][2] as u32;
    }

    println!("{}", ans);
}
