#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s1: String,
        s2: String,
    }

    println!(
        "{}",
        if s1 == ".#" && s2 == "#." || s2 == ".#" && s1 == "#." {
            "No"
        } else {
            "Yes"
        }
    )
}
