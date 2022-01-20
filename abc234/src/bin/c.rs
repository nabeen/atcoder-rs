#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        k: usize
    }

    let ans = format!("{:b}", k).to_string().replace("1", "2");
    println!("{}", ans);
}
