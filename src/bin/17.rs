fn main() {
    let a: Vec<usize> = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ]
    .iter()
    .map(|s| s.chars().count())
    .collect();
    let b: Vec<usize> = [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nighteen",
    ]
    .iter()
    .map(|s| s.chars().count())
    .collect();
    let c: Vec<usize> = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .map(|s| s.chars().count())
    .collect();
    let ans: usize = (1..=1000)
        .map(|x| {
            let mut len = 0;
            if x >= 1000 {
                len += "one".chars().count() + "thousand".chars().count();
                return len;
            }
            if x >= 100 {
                len += c[x % 1000 / 100] + "hundred".chars().count();
            }
            if x >= 100 && x % 100 > 0 {
                len += "and".chars().count();
            }
            if x % 100 > 0 {
                let ten = x % 100 / 10;

                if ten >= 2 {
                    // 20 ~ 99
                    len += a[ten] + c[x % 10]
                } else if ten >= 1 {
                    // 10 ~ 19
                    len += b[x % 100 - 10];
                } else {
                    // 0 ~ 9
                    len += c[x % 10];
                }
            }
            len
        })
        .sum();

    println!("{}", ans);
}
