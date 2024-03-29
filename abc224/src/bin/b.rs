#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize;w]; h]
    }

    let mut ans = "Yes";
    for i1 in 0..h {
        for i2 in i1 + 1..h {
            for j1 in 0..w {
                for j2 in j1 + 1..w {
                    if !(a[i1][j1] + a[i2][j2] <= a[i2][j1] + a[i1][j2]) {
                        ans = "No";
                        break;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
