use std::collections::HashMap;

const N: usize = 1_000_000;

fn f(n: usize, set: &mut HashMap<usize, usize>) -> usize {
    if let Some(&len) = set.get(&n) {
        len
    } else {
        let len = 1 + if n % 2 == 0 {
            f(n / 2, set)
        } else {
            f(3 * n + 1, set)
        };
        set.insert(n, len);
        len
    }
}

fn main() {
    let mut set: HashMap<usize, usize> = HashMap::new();
    set.insert(1, 1);
    println!("{}", (1..N).max_by_key(|&x| f(x, &mut set)).unwrap());
}
