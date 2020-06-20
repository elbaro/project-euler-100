const REACH_1: bool = false;
const REACH_89: bool = true;

fn f(a: &mut Vec<Option<bool>>, num: usize) -> bool {
    if num < 600 && a[num].is_some() {
        return a[num].unwrap();
    }

    let mut sum = 0;
    let mut n = num;
    while n > 0 {
        let digit = n % 10;
        sum += digit * digit;
        n /= 10;
    }

    let ret = f(a, sum);

    if num < 600 {
        a[num] = Some(ret);
    }

    ret
}

fn main() {
    let mut ans = 0;
    let mut a: Vec<Option<bool>> = vec![None; 600];
    a[1] = Some(REACH_1);
    a[89] = Some(REACH_89);

    for i in 2..10_000_000 {
        if f(&mut a, i) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
