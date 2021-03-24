#![allow(dead_code)]
#![allow(unused)]
use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    }
    let ans = max(b - c, b - d);
    println!("{}", ans);
}
