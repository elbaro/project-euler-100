use std::collections::VecDeque;

fn main() {
    const N: u32 = 12_000;

    let mut q: VecDeque<((u32, u32), (u32, u32))> = VecDeque::new();
    q.push_back(((1, 3), (1, 2)));

    let mut ans = 0;

    while let Some(((a, b), (c, d))) = q.pop_front() {
        let f = b + d;
        if f <= N {
            ans += 1;
            let e = a + c;
            q.push_back(((a, b), (e, f)));
            q.push_back(((e, f), (c, d)));
        }
    }

    println!("{}", ans);
}
