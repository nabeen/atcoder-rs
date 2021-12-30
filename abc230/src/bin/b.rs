#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        s: String
    }

    let t = "oxx".repeat(10 ^ 5);
    println! {"{}", if t.find(&s).is_some() {"Yes"} else {"No"}}
}
