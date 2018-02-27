extern crate chrono;
extern crate time;

use chrono::prelude::*;

fn main() {
    let mut d = Utc.ymd(1901, 1, 1);
    let e = Utc.ymd(2000, 12, 31);
    let mut ans = 0;
    loop {
        if d.day()==1 && d.weekday() == Weekday::Sun {
            ans += 1;
        }
        if d == e { break; }
        d = d + time::Duration::days(1);
    }
    println!("{}", ans);
}
