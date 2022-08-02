#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        UV: [(Usize1,Usize1);M]
    }

    let mut edges = vec![vec![false;N];N]; 
    for (u,v) in UV {
        edges[u][v] = true;
        edges[v][u] = true;
    }

    let mut ans = 0;
    for i in 0..N {
        for j in (i+1)..N {
            for k in (j+1)..N {
                if edges[i][j] && edges[j][k] && edges[k][i] {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans)
}
