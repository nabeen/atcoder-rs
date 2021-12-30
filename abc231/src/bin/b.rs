#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: isize,
        s: [String; n]
    }

    let mut map = HashMap::new();
    for name in s {
        *map.entry(name).or_insert(0) += 1;
    }

    println!("{}", map.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap().0);
}
