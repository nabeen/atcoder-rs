#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
    }

    let mut all = vec![
        "ABC".to_string(),
        "ARC".to_string(),
        "AGC".to_string(),
        "AHC".to_string(),
    ];
    all.sort();
    let mut s = vec![s1, s2, s3];
    s.sort();

    let mut ans = &all[3];
    for (i, _k) in all.iter().enumerate() {
        if i < s.len() && all[i] != s[i] {
            // println!("{}", all[i]);
            ans = &all[i];
            break;
        }
    }

    println!("{}", ans)
}
