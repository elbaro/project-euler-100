#![feature(vec_resize_default)]
use reqwest;

fn main() {
    // solved by hand
    let res = reqwest::blocking::get("https://projecteuler.net/project/resources/p082_matrix.txt")
        .unwrap()
        .text()
        .unwrap();
    let n = 80;
    let a: Vec<Vec<i32>> = res
        .lines()
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();
    let mut d: Vec<Vec<i32>> = Vec::with_capacity(n);
    for i in 0..n {
        d.push(Vec::with_capacity(n));
        d[i].resize(n, Default::default());
    }

    for i in 0..n {
        d[i][0] = a[i][0];
    }
    for j in 1..n {
        let mut s = 0;
        for i in 0..n {
            if i == 0 {
                s = d[i][j - 1] + a[i][j];
                d[i][j] = s;
            } else {
                s = std::cmp::min(d[i][j - 1], s) + a[i][j];
                d[i][j] = s;
            }
        }
        for i in (0..n - 1).rev() {
            d[i][j] = std::cmp::min(d[i][j], d[i + 1][j] + a[i][j]);
        }
    }
    let ans = (0..n).map(|i| d[i][n - 1]).min().unwrap();
    println!("{}", ans);
}
