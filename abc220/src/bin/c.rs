#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: usize,
    }

    // 一旦aのシグマを計算しておく
    let sum: usize = a.iter().sum();

    let p = x / sum;
    let mut sumb = p * sum;
    let mut ans = p * n;

    for v in a {
        sumb += v;
        ans += 1;
        if sumb > x {
            break;
        }
    }
    println!("{}", ans);
}
