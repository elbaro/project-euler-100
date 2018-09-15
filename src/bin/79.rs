extern crate reqwest;

fn main() {
    // solved by hand
    println!("73162890");
    // let res = reqwest::get("https://projecteuler.net/project/resources/p079_keylog.txt").unwrap().text().unwrap();
    // let a:Vec<&str> = res.lines().collect();

    // for i in 1..1_000_000 {
    //     let s = i.to_string();
    //     // let s = "73162890".to_string();
    //     let s = s.as_bytes();
    //     let mut ok = true;

    //     for j in &a {
    //         let j = j.as_bytes();
    //         let mut k = 0;

    //         for d in s {
    //             if *d==j[k] {
    //                 k += 1;
    //                 if k==3 {
    //                     break;
    //                 }
    //             }
    //         }
    //         if k!=3 {
    //             ok = false;
    //             break;
    //         }
    //     }
    //     if ok {
    //         println!("{}", i);
    //         break;
    //     }
    // }
}
