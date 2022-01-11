#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: String
    }

    let s = &s[n - 1..n];
    println!("{}", if s == "o" { "Yes" } else { "No" })
}
