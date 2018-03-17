fn main() {
    // x ~ 6x consist of the same digits
    // -> all equal modular 9
    // x%9 == 2x%9 -> x%9==0
    for i in 1.. {
        let x = i * 9;
        let mut xs:Vec<_> = x.to_string().chars().collect();
        xs.sort_unstable();
        let mut cnt = 0;
        for k in 2..7 {
            let y = x * k;
            let mut ys:Vec<_> =y.to_string().chars().collect();
            ys.sort_unstable();
            if xs!=ys {
                break;
            }
            cnt += 1;
        }
        if cnt==5 {
            println!("{}", x);
            break;
        }
    }
}
