#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::input;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Person {
    m: String,
    n: String,
}

fn main() {
    input! {
        n: usize,
        st: [(String, String);n]
    }

    let map: HashSet<Person> = st
        .iter()
        .map(|p| Person {
            n: p.0.clone(),
            m: p.1.clone(),
        })
        .collect();

    println!("{}", if map.len() == st.len() { "No" } else { "Yes" });
}
