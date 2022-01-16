#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
    }

    println!("{}", a ^ b)
}
