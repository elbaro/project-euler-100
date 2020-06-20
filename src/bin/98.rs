fn main() {
    let txt = reqwest::blocking::get("https://projecteuler.net/project/resources/p098_words.txt")
        .unwrap()
        .text()
        .unwrap();

    let words: Vec<String> = txt
        .split(",")
        .map(|word| word[1..word.len() - 1].to_string())
        .collect();

    // the maxmimum len of an anagram pair is 9
    let mut ans = 0i32;
    for i in 0..words.len() {
        let mut w1: Vec<u8> = words[i].bytes().collect();
        w1.sort();
        for j in i + 1..words.len() {
            if words[i].len() != words[j].len() {
                continue;
            }

            let mut w2: Vec<u8> = words[j].bytes().collect();
            w2.sort();
            if w1 == w2 {
                for num in 1.. {
                    // num*num == w1
                    let sq = num * num;
                    if sq > 1_000_000_000 {
                        break;
                    }
                    let mut m = sq;
                    let mut invalid = false;
                    let mut assigns = [0u8; 10]; // 0=>'G' 1=>'D' ..
                    for idx in (0..words[i].len()).rev() {
                        if m == 0 {
                            invalid = true;
                            break;
                        }
                        let digit = m % 10;
                        let ch = words[i].as_bytes()[idx]; // 'A'~'Z'
                        if assigns[digit] == 0 {
                            assigns[digit] = ch;
                        } else if assigns[digit] != ch {
                            invalid = true;
                            break;
                        }
                        m /= 10;
                    }
                    if m > 0 {
                        invalid = true;
                    }
                    // check no two digits map to the same letter
                    for k in 0..10 {
                        if assigns[k] == 0 {
                            continue;
                        }
                        for l in k + 1..10 {
                            if assigns[k] == assigns[l] {
                                invalid = true;
                                break;
                            }
                        }
                        if invalid {
                            break;
                        }
                    }
                    if invalid {
                        continue;
                    }

                    let mut num2 = 0i32;
                    for b in words[j].as_bytes() {
                        let digit = assigns.iter().position(|value| value == b).unwrap() as i32;
                        num2 = num2 * 10 + digit;
                        if num2 == 0 {
                            // leading zero not allowed
                            invalid = true;
                            break;
                        }
                    }
                    if invalid {
                        continue;
                    }
                    let rt = (num2 as f64).sqrt().round() as i32;
                    if num2 == rt * rt {
                        if (num * num) as i32 > ans {
                            ans = (num * num) as i32;
                        }
                        if num2 > ans {
                            ans = num2;
                        }
                        // println!("{} {} {} {}", words[i], num * num, words[j], num2);
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
