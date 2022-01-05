#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use regex::Regex;

fn main() {
    input! {
        s: String,
    }

    println!(
        "{}",
        if Regex::new(r"er$").unwrap().is_match(&s) {
            "er"
        } else {
            "ist"
        }
    )
}
