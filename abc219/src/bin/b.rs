#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
        t: Chars
    }

    let mut s = String::new();
    for i in t.iter() {
        match i {
            '1' => s += &s1,
            '2' => s += &s2,
            '3' => s += &s3,
            _ => (),
        }
    }

    println!("{}", s)
}
