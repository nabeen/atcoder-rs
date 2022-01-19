#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::iter::FromIterator;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        abc: Chars,
    }
    let a = abc[0];
    let b = abc[1];
    let c = abc[2];

    let abc: usize = String::from_iter(&[a, b, c]).parse().unwrap();
    let bca: usize = String::from_iter(&[b, c, a]).parse().unwrap();
    let cab: usize = String::from_iter(&[c, a, b]).parse().unwrap();

    println!("{}", abc + bca + cab);
}
