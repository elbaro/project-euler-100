#![feature(inclusive_range_syntax, io)]
extern crate reqwest;

fn main() {
    let res = reqwest::get("https://projecteuler.net/project/resources/p022_names.txt").unwrap().text().unwrap();
    let names : Vec<String> = {
        let mut vec:Vec<String> = res.split(',').map(|s| (&s[1..s.len()-1]).to_string()).collect();
        vec.sort_unstable();
        vec
    };
    let ans = names.iter().enumerate().map(|(index,s)| {
        (index+1)*s.chars().map(|ch| (ch as usize)-('A' as usize)+1).sum::<usize>()
    }).sum::<usize>();
    println!("{}", ans);
}
