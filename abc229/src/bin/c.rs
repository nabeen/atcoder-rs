#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort_by(|x, y| y.0.cmp(&x.0));

    let mut ans = 0;
    let mut gram = 0;
    for (a, b) in ab {
        if gram + b <= w {
            ans += a * b;
            gram += b;
        } else {
            loop {
                if gram + 1 > w {
                    break;
                } else {
                    ans += a;
                    gram += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
