use reqwest;

fn main() {
    let res =
        reqwest::blocking::get("https://projecteuler.net/project/resources/p067_triangle.txt")
            .unwrap()
            .text()
            .unwrap();
    let tri: Vec<Vec<usize>> = res
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    const N: usize = 100;
    assert_eq!(tri.len(), N);
    let mut s = [[0; N]; N];
    s[0][0] = tri[0][0];
    for i in 1..N {
        s[i][0] = s[i - 1][0] + tri[i][0];
        for j in 1..=i {
            s[i][j] = std::cmp::max(s[i - 1][j - 1], s[i - 1][j]) + tri[i][j];
        }
    }

    println!("{:}", s[N - 1].iter().max().unwrap());
}
