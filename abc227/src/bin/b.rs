#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
    }

    // 4ab+3a+3b
    let mut set: HashSet<usize> = HashSet::new();
    for a in 1..150 {
        for b in 1..150 {
            if 4 * a * b + 3 * a + 3 * b <= 1000 {
                set.insert(4 * a * b + 3 * a + 3 * b);
            } else {
                break;
            }
        }
    }
    let mut ans = 0;
    for v in s {
        if !set.contains(&v) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
