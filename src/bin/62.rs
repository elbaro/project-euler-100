use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    let mut first = HashMap::new();

    for i in 1u64..10000 {
        let cube = i * i * i;
        let mut s = cube.to_string().into_bytes();
        s.sort();

        let mut x = 1u64;
        for &d in &s {
            x = x * 10 + (d - '0' as u8) as u64;
        }
        first.entry(x).or_insert(cube);
        let e = m.entry(x).or_insert(0u32);
        *e += 1;
    }
    let ans = m.into_iter().filter(|&(_k,v)| v==5).map(|(k,_v)| first.get(&k).unwrap()).min().unwrap();
    println!("{}", ans);
}