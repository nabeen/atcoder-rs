#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        n: u64
    }

    let mut k = 1;
    loop {
        if 2_u64.pow(k) <= n {
            k += 1;
            continue;
        } else {
            break;
        }
    }
    println!("{}", k - 1)
}
