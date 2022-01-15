#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        s: String
    }

    println!("{}", if s == "Hello,World!" { "AC" } else { "WA" });
}
