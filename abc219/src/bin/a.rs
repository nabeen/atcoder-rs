#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    match x {
        0..=39 => println!("{}", 40 - x),
        40..=69 => println!("{}", 70 - x),
        70..=89 => println!("{}", 90 - x),
        _ => println!("expert"),
    }
}
