#![allow(non_snake_case)]
#![allow(unused_imports)]

use num::abs;
use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }

    let diff = a
        .iter()
        .zip(b.iter())
        .fold(0, |acc, x| acc + (x.0 - x.1).abs());

    println!(
        "{}",
        if k >= diff && (k - diff) % 2 == 0 {
            "Yes"
        } else {
            "No"
        }
    )
}
