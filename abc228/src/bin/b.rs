#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let mut set: HashSet<usize> = HashSet::new();
    let mut friend = x;
    loop {
        if set.contains(&friend) {
            break;
        }
        set.insert(friend);
        friend = a[friend - 1];
    }
    println!("{}", set.len());
}
