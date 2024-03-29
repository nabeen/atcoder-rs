#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        s: isize,
        t: isize,
    }

    let mut ans = 0;
    for a in 0..101 {
        for b in 0..101 {
            for c in 0..101 {
                if a + b + c <= s && a * b * c <= t {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
