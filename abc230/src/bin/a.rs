#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        n: i8
    }

    println!("AGC{0:>03}", if n >= 42 { n + 1 } else { n })
}
