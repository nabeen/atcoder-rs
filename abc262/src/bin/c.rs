#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::cmp::{min, max};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1;n]
    }

    let mut ans: i64 = 0;
    let mut c: i64 = 0;
    for i in 0..n {
        if a[i] == i {
            c+=1;
        } else if a[a[i]] == i && a[i] > i {
            ans+=1;
        }
    }

    ans += c * (c - 1) / 2;
    println!("{}", ans);
}
