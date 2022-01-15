#![allow(non_snake_case)]
#![allow(unused_imports)]
use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [[f64;2];n]
    }

    let mut max = 0_f64;
    for i in 0..xy.len() - 1 {
        for j in i..xy.len() {
            let a = &xy[i];
            let b = &xy[j];
            let tmp = ((a[0] - b[0]).abs().powi(2) + (a[1] - b[1]).abs().powi(2)).sqrt();
            max = if tmp > max { tmp } else { max }
        }
    }
    println!("{}", max);
}
