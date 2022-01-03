#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        l: [[usize]; n]
    }

    let mut set = HashSet::new();
    for v in l {
        set.insert(v);
    }
    println!("{}", set.len());
}
