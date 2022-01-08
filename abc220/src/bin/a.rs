#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize
    }

    let mut ans: isize = -1;
    for i in 1..1000 {
        if a <= c * i && c * i <= b {
            ans = c * i;
            break;
        }
    }

    println!("{}", ans);
}
