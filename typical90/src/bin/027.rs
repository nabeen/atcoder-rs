#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    // println!("{:?}", s)
    let mut map = HashMap::new();
    for (i, k) in s.iter().enumerate() {
        if map.contains_key(k) {
            continue;
        } else {
            map.insert(k, true);
            println!("{}", i + 1);
        }
    }
}
