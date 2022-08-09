#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        cp: [(usize, usize);n],
        q: usize,
        lr: [(usize, usize);q],
    }

    // 累積和保持用
    let mut s1 = vec![0; n + 1];
    let mut s2 = vec![0; n + 1];

    // 累積和
    for (i, &(c, p)) in cp.iter().enumerate() {
        if c == 1 {
            s1[i + 1] = s1[i] + p;
            s2[i + 1] = s2[i];
        } else if c == 2 {
            s1[i + 1] = s1[i];
            s2[i + 1] = s2[i] + p;
        }
    }

    for (l, r) in lr {
        println!("{} {}", s1[r] - s1[l - 1], s2[r] - s2[l - 1]);
    }
}
