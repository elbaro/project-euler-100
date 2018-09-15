const n:usize = 100;
fn main() {
    let mut d=vec![0u32;n+5];

    d[0]=1;
    for i in 1..=n {
        for j in (i..=n) {
            d[j] += d[j-i];
        }
    }
    
    println!("{}", d[n]-1);
}
