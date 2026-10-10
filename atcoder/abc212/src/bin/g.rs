use cplib::num::fp::fp;
use proconio::input;

const P: u32 = 998_244_353;

fn main() {
    input! {
        p: u64,
    };

    let mut d = vec![];
    for i in 1.. {
        if i * i > p - 1 {
            break;
        }
        if (p - 1) % i == 0 {
            d.push(i);
            if (p - 1) / i != i {
                d.push((p - 1) / i);
            }
        }
    }
    d.sort();

    let mut cnt = vec![0; d.len()];
    for (cnt, d) in cnt.iter_mut().zip(d.iter()) {
        *cnt = (p - 1) / d;
    }
    for i in (0..d.len()).rev() {
        for j in i + 1..d.len() {
            if d[j] % d[i] == 0 {
                cnt[i] -= cnt[j];
            }
        }
    }

    let mut ans = fp!(1, mod P);
    for (d, cnt) in d.iter().zip(cnt.iter()) {
        ans += fp!((p - 1) / d * cnt);
    }
    println!("{ans}");
}
