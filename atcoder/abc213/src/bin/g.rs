use cplib::algebra::canonical::Canonical;
use cplib::bitwise::set_power_series::elementary::sps_log;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

use cplib::num::fp::fp;

const P: u32 = 998_244_353;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut f = vec![fp!(1, mod P); 1 << n];
    for &(a, b) in &ab {
        for v in 0..1 << n {
            if (v >> a) & 1 == 1 && (v >> b) & 1 == 1 {
                f[v] *= fp!(2);
            }
        }
    }

    let g = sps_log(&Canonical::new(), &f);
    let mut ans = vec![fp!(0); n];
    for v in 0..1 << n {
        for i in 1..n {
            if v & 1 == 1 && (v >> i) & 1 == 1 {
                ans[i] += g[v];
            }
        }
    }
    println!("{}", ans[1..].iter().join("\n"));
}
