#![feature(vec_resize_default)]
extern crate reqwest;

fn main() {
    // solved by hand
    let res = reqwest::get("https://projecteuler.net/project/resources/p081_matrix.txt")
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
        d[i].resize_default(n);
    }

    d[0][0] = a[0][0];
    for i in 1..n {
        d[0][i] = d[0][i - 1] + a[0][i];
    }
    for i in 1..n {
        d[i][0] = d[i - 1][0] + a[i][0];
        for j in 1..n {
            d[i][j] = std::cmp::min(d[i - 1][j], d[i][j - 1]) + a[i][j];
        }
    }
    println!("{}", d[n - 1][n - 1]);
}
