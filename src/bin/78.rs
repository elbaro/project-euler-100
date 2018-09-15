const N:usize = 100000;
const MOD:u32 = 1_000_000;
fn main() {
    let mut d=vec![0u32;N+5];

    d[0]=1;
    for i in 1..=N {
        for j in (i..=N) {
            d[j] = (d[j]+d[j-i])%MOD;
        }
    }

    let mut ans = 0;
    for i in 1..=N {
        if d[i] == 0 {
            ans = i;
            break;
        }
    }
    
    println!("{}", ans);
}
