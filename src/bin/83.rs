#![feature(vec_resize_default)]
extern crate reqwest;
use std::collections::VecDeque;

fn main() {
    // solved by hand
    let res = reqwest::get("https://projecteuler.net/project/resources/p083_matrix.txt").unwrap().text().unwrap();
    let n = 80;
    let oo = 10000 * 80 * 2;
    let dy = [0,1,0,-1];
    let dx = [1,0,-1,0];
    let a:Vec<Vec<i32>> = res.lines().map(|line| line.split(',').map(|s| s.parse().unwrap()).collect()).collect();
    let mut in_q:Vec<Vec<bool>> = Vec::with_capacity(n);
    let mut d:Vec<Vec<i32>> = Vec::with_capacity(n);
    for i in 0..n {
        d.push(Vec::with_capacity(n));
        d[i].resize(n, oo);

        in_q.push(Vec::with_capacity(n));
        in_q[i].resize(n, false);
    }

    // SPFA
    let mut q : VecDeque<(usize,usize)> = VecDeque::new();
    q.push_back((0,0));
    in_q[0][0] = true;
    d[0][0] = a[0][0];

    while let Some(v) = q.pop_front() {
        in_q[v.1][v.0] = false;

        for k in 0..4 {
            let (x, y) = (v.0 as i32 + dx[k], v.1 as i32 + dy[k]);
            if x<0 || x==n as i32 || y<0 || y==n as i32 {
                continue;
            }
            let (x,y) = (x as usize, y as usize);
        
            if d[y][x]>d[v.1][v.0]+a[y][x] {
                d[y][x] = d[v.1][v.0]+a[y][x];

                if !in_q[y][x] {
                    q.push_back((x,y));
                    in_q[y][x] = true;
                }
            }
        }
    }
    println!("{}", d[n-1][n-1]);
}
