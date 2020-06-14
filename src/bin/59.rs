extern crate reqwest;

fn main() {
    let res = reqwest::get("https://projecteuler.net/project/resources/p059_cipher.txt")
        .unwrap()
        .text()
        .unwrap();
    let t: Vec<u8> = res
        .split(',')
        .map(|token| token.trim().parse::<u8>().unwrap())
        .collect();

    let mut ans: u32 = 0;
    'find: for a in 'a' as u8..('z' as u8) + 1 {
        for b in 'a' as u8..('z' as u8) + 1 {
            for c in 'a' as u8..('z' as u8) + 1 {
                let key = [a, b, c];
                let decrypted: Vec<u8> =
                    t.iter().enumerate().map(|(i, t)| t ^ key[i % 3]).collect();

                if decrypted.iter().any(|&ord| ord < 32 || ord >= 127) {
                    continue;
                }
                let mut txt: String = decrypted.iter().map(|&u| u as char).collect();
                txt.make_ascii_lowercase();
                if txt.contains("the ") && txt.contains("to") {
                    ans = decrypted.iter().map(|&byte| byte as u32).sum();
                    break 'find;
                }
            }
        }
    }
    println!("{}", ans);
}
