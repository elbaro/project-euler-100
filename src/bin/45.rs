#![feature(inclusive_range_syntax, io)]

fn nth_hexa(n: usize) -> usize {
    n * (2 * n - 1)
}

fn nth_penta(n: usize) -> usize {
    n * (3 * n - 1) / 2
}

fn is_penta(x: usize) -> bool {
    // 3(n-1)^2 < 2x = n(3n-1) < 3n^2
    let n = ((x * 2 / 3) as f32).sqrt() as usize + 1;
    x == nth_penta(n)
}

fn is_tri(x: usize) -> bool {
    // n^2 < 2x = n(n+1) < (n+1)^2
    let n = ((x * 2) as f32).sqrt() as usize;
    n > 0 && x == n * (n + 1) / 2
}

fn main() {
    let ans = nth_hexa(
        (144..)
            .filter(|&n| {
                let x = nth_hexa(n);
                is_tri(x) && is_penta(x)
            })
            .next()
            .unwrap(),
    );
    println!("{}", ans);
}
