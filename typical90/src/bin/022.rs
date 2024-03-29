#![allow(non_snake_case)]
#![allow(unused_imports)]
use num::integer::gcd;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let r = gcd(a, gcd(b, c));
    println!("{}", (a / r - 1) + (b / r - 1) + (c / r - 1))
}
