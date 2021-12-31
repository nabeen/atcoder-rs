#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s1: Chars,
        s2: Chars
    }

    let b: char = '#';

    println!(
        "{}",
        if s1[0] == b && s1[1] == b {
            "Yes"
        } else if s2[0] == b && s2[1] == b {
            "Yes"
        } else if s1[0] == b && s2[0] == b {
            "Yes"
        } else if s1[1] == b && s2[1] == b {
            "Yes"
        } else {
            "No"
        }
    )
}
