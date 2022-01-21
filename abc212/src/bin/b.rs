#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::collections::HashSet;
use std::iter::FromIterator;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: Chars,
    }

    let mut ans = "Strong";
    // 1st
    let uniq: HashSet<&char> = HashSet::from_iter(x.iter());
    if uniq.len() == 1 {
        ans = "Weak";
    }
    let mut plus = true;
    for i in 0..x.len() - 1 {
        if (x[i].to_string().parse::<usize>().unwrap() + 1) % 10
            == x[i + 1].to_string().parse::<usize>().unwrap()
        {
        } else {
            plus = false;
        }
    }
    if plus {
        ans = "Weak";
    }

    println!("{}", ans);
}
