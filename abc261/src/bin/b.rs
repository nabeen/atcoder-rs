#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars;n]
    }

    for i in 0..n {
        for j in i+1..n {
            if a[i][j] == 'W' {
                if a[j][i] != 'L' {
                    println!("incorrect");
                    return    
                }
            }
            if a[i][j] == 'L' {
                if a[j][i] != 'W' {
                    println!("incorrect");
                    return    
                }
            }
            if a[i][j] == 'D' {
                if a[j][i] != 'D' {
                    println!("incorrect");
                    return    
                }
            }
        }
    }
    println!("correct");
}
