#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    println!(
        "{}",
        if 0 < a && b == 0 {
            "Gold"
        } else if a == 0 && 0 < b {
            "Silver"
        } else if 0 < a && 0 < b {
            "Alloy"
        } else {
            ""
        }
    )
}
