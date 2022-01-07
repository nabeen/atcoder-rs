#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32
    }

    let base: u32 = 32;
    println!("{}", base.pow(a - b))
}
