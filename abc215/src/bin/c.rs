#![allow(non_snake_case)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        s: String,
        k: usize,
    }

    let mut hash = HashSet::new();
    for perm in s.chars().permutations(s.len()) {
        hash.insert(perm.iter().join(""));
    }
    let mut dic = hash.into_iter().collect::<Vec<String>>();
    dic.sort();

    println!("{}", dic[k - 1]);
}
