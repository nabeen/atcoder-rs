#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut s: Vec<(_, _)> = a.iter().enumerate().map(|(i, v)| (i + 1, v)).collect();
    s.sort_by(|x, y| x.1.cmp(y.1));
    println!("{:?}", s[n - 2].0);
}
