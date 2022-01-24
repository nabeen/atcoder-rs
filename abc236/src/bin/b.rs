#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; 4*n-1]
    }

    let total = (1..n + 1).fold(0, |sum, x| sum + 4 * x);
    let sum = a.iter().fold(0, |sum, x| sum + x);
    println!("{}", total - sum);
}
