#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        n: i16,
        k: i16,
        a: i16,
    }

    let mut now = a;
    for _i in 1..k {
        if n == now {
            now = 1;
        } else {
            now += 1;
        }
    }
    println!("{}", now)
}
