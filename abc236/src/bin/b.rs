#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        mut a: [usize; 4*n-1]
    }

    let mut map = HashMap::new();
    for i in a {
        *map.entry(i).or_insert(0) += 1;
    }
    println!("{}", map.iter().min_by(|x, y| x.1.cmp(y.1)).unwrap().0)
}
