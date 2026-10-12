use cplib::collections::range_map::RangeMap;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        t: usize,
    };

    'L: for _ in 0..t {
        input! {
            n: usize,
            mut lr: [(Usize1, usize); n],
        }
        lr.sort_by_key(|x| (x.1, !x.0));
        let mut map = RangeMap::new();
        for &(l, r) in &lr {
            let k = map.get(l).map_or(l, |k| k.1);
            if r as u64 <= k as u64 {
                println!("No");
                continue 'L;
            }
            map.assign(k, k + 1, ());
            map.merge(k);
        }
        println!("Yes");
    }
}
