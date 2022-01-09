#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
        t: Chars
    }

    if s == t {
        println!("Yes");
        return;
    }

    for i in 0..(s.len() - 1) {
        s.swap(i, i + 1);
        if s == t {
            println!("Yes");
            return;
        }
        s.swap(i, i + 1);
    }
    println!("No");
}
