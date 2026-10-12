use cplib::graph::min_cost_flow::MinCostFlow;
use cplib::graph::scc::Scc;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: i64,
        ab: [(Usize1, Usize1); m],
        x: [i64; n],
    };
    let mut e = vec![vec![]; n];
    for &(a, b) in &ab {
        e[a].push(b);
    }
    let scc = Scc::from_adjacency(&e);
    let l = scc.len();
    let mut y = vec![0; l];
    for i in 0..l {
        for &v in scc.group(i) {
            y[i] += x[v];
        }
    }

    let mut mcf = MinCostFlow::new(2 * l + 2);
    let s = 2 * l;
    let t = 2 * l + 1;

    mcf.add_edge(s, scc.component(0), k, 0);
    for i in 0..l {
        mcf.add_edge(i, l + i, 1, -y[i]);
        mcf.add_edge(i, l + i, k, 0);
        mcf.add_edge(l + i, t, k, 0);
    }

    for (i, e) in scc.condensation().iter().enumerate() {
        for j in e {
            mcf.add_edge(l + i, *j, k, 0);
        }
    }

    let ans = mcf.flow_limit(s, t, k).1;

    println!("{}", -ans);
}
