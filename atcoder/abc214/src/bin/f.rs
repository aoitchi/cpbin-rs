use proconio::input;
use proconio::marker::Bytes;

use cplib::num::fp::fp;

const P: u32 = 1_000_000_007;

fn main() {
    input! {
        s: Bytes,
    };
    let n = s.len();
    let mut f = vec![fp!(1, mod P)];
    let mut g = vec![fp!(0, mod P), fp!(1)];
    let mut prev = [n; 26];
    prev[(s[0] - b'a') as usize] = 0;
    for (i, c) in s.iter().enumerate().skip(1) {
        let c = (c - b'a') as usize;
        let a = if prev[c] == n || prev[c] == 0 {
            0
        } else {
            prev[c] - 1
        };
        let b = i - 1;
        let x =
            if a < b { g[b] - g[a] } else { fp!(0) } + if prev[c] == n { fp!(1) } else { fp!(0) };
        f.push(x);
        g.push(g.last().unwrap() + x);
        prev[c] = i;
    }
    println!("{}", g.last().unwrap());
}
