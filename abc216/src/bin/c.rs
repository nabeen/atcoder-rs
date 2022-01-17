#![allow(non_snake_case)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut n: usize
    }

    // let mut now = n;
    let mut ans = vec![];
    while n != 0 {
        if n % 2 == 0 {
            n /= 2;
            ans.push('B');
        } else {
            n -= 1;
            ans.push('A');
        }
    }

    println!("{}", ans.into_iter().rev().join(""));
}
