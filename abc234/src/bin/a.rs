#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn f(t: usize) -> usize {
    t * t + 2 * t + 3
}

fn main() {
    input! {
        t: usize
    }

    println!("{}", f(f(f(t) + t) + f(f(t))))
}
