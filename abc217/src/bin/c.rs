#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }

    let mut p = p.iter().enumerate().collect::<Vec<_>>();
    p.sort_by_key(|&(_i, v)| v);
    p.iter().for_each(|&(i, x)| {
        if *x == 1 {
            print!("{}", i + 1)
        } else {
            print!(" {}", i + 1)
        }
    });
    println!("");
}
