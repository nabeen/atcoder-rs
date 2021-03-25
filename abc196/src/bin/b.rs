use proconio::input;

fn main() {
    input! {
        x: String,
    }
    let s: Vec<&str> = x.split(".").collect();
    println!("{}", s[0]);
}
