#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        x: f32,
    }

    println!("{}", x.round());
}
