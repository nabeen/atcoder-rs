#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }

    let mut set = HashSet::new();
    for v in t {
        set.insert(v);
    }
    for v in s {
        println!("{}", if set.contains(&v) { "Yes" } else { "No" })
    }
}
