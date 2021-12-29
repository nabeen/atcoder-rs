#![allow(non_snake_case)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    println!(
        "{}",
        if s.iter()
            .zip(t.iter())
            .map(|(&a, &b)| (a as i32 - b as i32).rem_euclid(26))
            .all_equal()
        {
            "Yes"
        } else {
            "No"
        }
    );
}
