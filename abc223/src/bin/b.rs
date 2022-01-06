#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        s: String
    }
    let mut vec: Vec<char> = s.chars().collect();

    let mut min = vec.clone();
    let mut max = vec.clone();

    for _ in 1..s.len() {
        vec.rotate_left(1);
        min = std::cmp::min(min, vec.clone());
        max = std::cmp::max(max, vec.clone());
    }

    println!("{}", min.into_iter().collect::<String>());
    println!("{}", max.into_iter().collect::<String>());
}
