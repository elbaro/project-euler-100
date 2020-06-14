#![feature(inclusive_range_syntax, io)]
extern crate reqwest;

fn main() {
    let res = reqwest::get("https://projecteuler.net/project/resources/p042_words.txt")
        .unwrap()
        .text()
        .unwrap();
    let words: Vec<String> = {
        let mut vec: Vec<String> = res
            .split(',')
            .map(|s| (&s[1..s.len() - 1]).to_string())
            .collect();
        vec.sort_unstable();
        vec
    };
    let ans = words
        .iter()
        .map(|s| {
            s.chars()
                .map(|ch| (ch as usize) - ('A' as usize) + 1)
                .sum::<usize>()
        })
        .filter(|t| {
            let n = ((2 * t) as f32).sqrt() as usize;
            2 * t == n * (n + 1)
        })
        .count(); // 2t = n(n+1)  ->  n<sqrt(2t)<n+1
    println!("{}", ans);
}
