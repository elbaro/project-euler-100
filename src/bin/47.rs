#![feature(iterator_step_by)]
extern crate primal;
//extern crate itertools;
//use itertools::Itertools;

fn main() {
    let sieve = primal::Sieve::new(1_000_000);
    let mut cnt = 0;
    for x in (2*3*5*7).. {
        if sieve.factor(x).unwrap().len()==4 {
            cnt += 1;
            if cnt == 4 {
                println!("{}", x-3);
                break;
            }
        } else {
            cnt = 0;
        }
    }
//    [slow] iterator version
//    let ans = (2*3*5*7..).map(|x| (x, sieve.factor(x).unwrap().len()==4))
//        .tuple_windows()
//        .filter(|&((_,a),(_,b),(_,c),(_,d))| {
//            a && b && c && d
//        })
//        .map(|((a,_),(_,_),(_,_),(_,_))| {
//            a
//        })
//        .next().unwrap();
//    println!("{}", ans);
}
