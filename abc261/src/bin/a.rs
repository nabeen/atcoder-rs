#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::cmp::{max, min};

use proconio::input;

fn main() {
    input! {
        l1: isize,
        r1: isize,
        l2: isize,
        r2: isize,
    }

    println!("{}", max(0, min(r1, r2) - max(l1, l2)))
}
