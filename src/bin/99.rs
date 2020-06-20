fn main() {
    let txt =
        reqwest::blocking::get("https://projecteuler.net/project/resources/p099_base_exp.txt")
            .unwrap()
            .text()
            .unwrap();
    let nums: Vec<(usize, usize)> = txt
        .lines()
        .map(|s| {
            let mut it = s.split(",");
            (
                it.next().unwrap().parse::<usize>().unwrap(),
                it.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();
    let mut idx = 0;
    let mut mx = 0f64;
    for i in 0..nums.len() {
        let (a, b) = nums[i];
        let value = (b as f64) * (a as f64).ln();
        if i == 0 || value > mx {
            mx = value;
            idx = i;
        }
    }
    println!("{}", idx + 1);
}
