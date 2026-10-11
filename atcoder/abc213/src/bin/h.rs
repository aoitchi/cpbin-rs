use cplib::algebra::canonical::Canonical;
use cplib::divide_and_conquer::cdq;
use cplib::num::fp::{Fp, fp};
use cplib::poly::poly_convolve;
use proconio::input;
use proconio::marker::Usize1;

const P: u32 = 998_244_353;

fn main() {
    input! {
        n: usize,
        m: usize,
        t: usize,
    };
    let mut e = vec![vec![m + 1; n]; n];
    let mut ps = vec![];
    for j in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
            p: [u32; t],
        }
        let mut p: Vec<Fp<P>> = p.into_iter().map(|p| fp!(p)).collect();
        p.insert(0, fp!(0));
        e[a][b] = j;
        e[b][a] = j;
        ps.push(p);
    }

    let mut state = (vec![vec![fp!(0); t + 1]; n], vec![vec![fp!(0); t + 1]; n]);
    for i in 0..n {
        (state.0)[i][0] = fp!(1);
    }
    cdq(
        t + 1,
        &mut state,
        |(f, acc), l, m, r| {
            for i in 0..n {
                for j in 0..n {
                    if i != j && e[j][i] < m {
                        let h = poly_convolve(
                            &Canonical::new(),
                            f[j][l..m].to_vec(),
                            ps[e[j][i]][1..r - l].to_vec(),
                        );
                        for k in m..r {
                            acc[i][k] += h[k - l - 1];
                        }
                    }
                }
            }
        },
        |(f, acc), k| {
            for i in 0..n {
                f[i][k] = acc[i][k];
            }
        },
    );
    println!("{}", state.0[0][t]);
}
