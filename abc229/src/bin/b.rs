#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut a: isize,
        mut b: isize,
    }

    let mut d = 0;
    loop {
        if a <= 0 || b <= 0 || d > 0 {
            break;
        }

        d = (a % 10 + b % 10) / 10;
        a /= 10;
        b /= 10;
    }
    println!("{}", if d > 0 { "Hard" } else { "Easy" });
}
