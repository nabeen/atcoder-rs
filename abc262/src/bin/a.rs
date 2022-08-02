#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        Y: isize
    }

    match Y % 4 {
        0 => println!("{}", Y + 2),
        1 => println!("{}", Y + 1),
        2 => println!("{}", Y + 0),
        3 => println!("{}", Y + 3),
        _ => (),
    }
}
