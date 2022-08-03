#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::cmp::{min, max};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if min(a[i], a[j]) == i+1 && max(a[i], a[j]) == j+1 && (1 <= i+1 && i+1 <= j+1 && j+1 <= n) {
                ans+=1;
            }
        }
    }
    println!("{}", ans);
}
