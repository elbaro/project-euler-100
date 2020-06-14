/**
 * I
 * II
 * III
 * IV
 * V
 * VI
 * VII
 * VIII
 * IX
 */

const TABLE: [u32; 256] = {
    let mut t = [0u32; 256];
    t['I' as usize] = 1;
    t['V' as usize] = 5;
    t['X' as usize] = 10;
    t['L' as usize] = 50;
    t['C' as usize] = 100;
    t['D' as usize] = 500;
    t['M' as usize] = 1000;
    t
};

const COST: [usize; 10] = [0, 1, 2, 3, 2, 1, 2, 3, 4, 2];

fn calc(line: &str) -> usize {
    let mut i = 0;
    let mut num = 0;
    let chars: Vec<u8> = line.bytes().collect();
    while i < line.len() {
        if i + 1 < line.len() && TABLE[chars[i] as usize] < TABLE[chars[i + 1] as usize] {
            num += TABLE[chars[i + 1] as usize] - TABLE[chars[i] as usize];
            i += 1;
        } else {
            num += TABLE[chars[i] as usize];
        }
        i += 1;
    }
    let mut compact = 0usize;
    let digits: Vec<u8> = num.to_string().bytes().collect();
    for (i, d) in digits.iter().enumerate() {
        if digits.len() - i < 4 {
            compact += COST[(d - '0' as u8) as usize];
        } else {
            // MMM.. cannot be replaced with others
            compact += (d - '0' as u8) as usize;
        }
    }

    line.len() - compact
}

fn main() {
    let data = reqwest::blocking::get("https://projecteuler.net/project/resources/p089_roman.txt")
        .unwrap()
        .text()
        .unwrap();
    let ans: usize = data.lines().map(|line| calc(line)).sum();
    println!("{}", ans);
}
