#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: isize,
        b: isize,
    }

    let mut kuri = 0;
    let mut aa = a;
    let mut bb = b;
    loop {
        if aa <= 0 || bb <= 0 || kuri > 0 {
            break;
        }

        kuri = (aa % 10 + bb % 10) / 10;
        aa /= 10;
        bb /= 10;
    }
    println!("{}", if kuri > 0 { "Hard" } else { "Easy" });
}
