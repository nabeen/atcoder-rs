#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        M: isize,
        H: isize,
    }
    if H%M == 0 {
        println!("Yes")
    } else {
        println!("No")
    }
}
