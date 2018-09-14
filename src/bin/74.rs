fn main() {
    let f = {
        let mut f = [1;10];
        for i in 1..=9 {
            f[i]=f[i-1]*i;
        }
        f
    };

    const N: usize = 1_000_000;
    const BURST: usize = 2_180_000;
    let mut visit = vec![0usize;BURST];
    let mut ans = 0;

    for i in 1..=N {
        let mut x = i;
        let mut chain = 0;

        while visit[x]!=i {
            visit[x] = i;
            chain += 1;

            let mut y = 0;
            while x>0 {
                y += f[x%10];
                x/=10;
            }
            x = y;
        }
        
        if chain == 60 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
