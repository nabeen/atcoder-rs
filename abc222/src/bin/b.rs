#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize; n]
    }

    let mut ans = 0;
    for v in a {
        if p > v {
            ans += 1;
        }
    }

    println!("{}", ans)
}
