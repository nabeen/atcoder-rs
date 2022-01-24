#![allow(non_snake_case)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        a: usize,
        b: usize,
    }

    s.swap(a - 1, b - 1);
    println!("{}", s.iter().join(""));
}
