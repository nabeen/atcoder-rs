#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        xy: String
    }

    let xy: Vec<_> = xy.split('.').collect();
    let x: usize = xy[0].parse().unwrap();
    let y: usize = xy[1].parse().unwrap();

    println!(
        "{}{}",
        x,
        match y {
            0..=2 => "-",
            3..=6 => "",
            7..=9 => "+",
            _ => "",
        }
    )
}
