#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        q: usize,
        tx: [(usize, usize); q]
    }

    let mut que = VecDeque::new();
    for (t, x) in tx {
        if t == 1 {
            que.push_front(x);
        }
        if t == 2 {
            que.push_back(x);
        }
        if t == 3 {
            println!("{}", que[x - 1]);
        }
    }
}
