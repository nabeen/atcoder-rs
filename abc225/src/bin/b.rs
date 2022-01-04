#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [[usize; 2]; n-1],
    }

    let mut map = HashMap::new();
    for v in a {
        *map.entry(v[0]).or_insert(0) += 1;
        *map.entry(v[1]).or_insert(0) += 1;
    }
    println!(
        "{}",
        if *map.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap().1 == n - 1 {
            "Yes"
        } else {
            "No"
        }
    );
}
