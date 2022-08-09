#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize;w];h]
    }

    let mut row = vec![0; h];
    let mut col = vec![0; w];

    for i in 0..h {
        for j in 0..w {
            row[i] += a[i][j];
            col[j] += a[i][j];
        }
    }

    for i in 0..h {
        let mut tmp: Vec<String> = vec![];
        for j in 0..w {
            tmp.push((row[i] + col[j] - a[i][j]).to_string())
        }
        println!("{}", tmp.join(" "));
    }
}
