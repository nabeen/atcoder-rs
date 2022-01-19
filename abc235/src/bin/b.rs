#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n]
    }

    let mut ans = 0;
    for i in h {
        if ans >= i {
            break;
        } else {
            ans = i;
        }
    }
    println!("{}", ans);
}
