#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        s: i8,
        t: i8,
        x: i8,
    }

    if s < t {
        println!("{}", if s <= x && x < t { "Yes" } else { "No" })
    } else if s > t {
        println!("{}", if s <= x || x < t { "Yes" } else { "No" })
    }
}
