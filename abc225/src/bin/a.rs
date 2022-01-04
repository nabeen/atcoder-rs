#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    if s[0] == s[1] && s[1] == s[2] {
        // 全て同じ
        println!("{}", 1);
    } else if s[0] == s[1] || s[1] == s[2] || s[2] == s[0] {
        // 2つ同じ
        println!("{}", 3);
    } else {
        // 全て異なる
        println!("{}", 6);
    }
}
