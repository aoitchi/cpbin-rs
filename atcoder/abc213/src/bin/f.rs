use cplib::sequence::suffix_array::SuffixArray;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        n: usize,
        s: Bytes,
    };

    let sa = SuffixArray::from_slice(&s);
    let lcp = sa.lcp();

    let mut ans = vec![0; n];
    let mut stack: Vec<(usize, usize)> = vec![];
    let mut x = 0;
    for i in 1..n {
        let mut len = 1;
        while stack.len() > 0 {
            if stack.last().unwrap().0 >= lcp[i - 1] {
                let (v, l) = stack.pop().unwrap();
                len += l;
                x -= v * l;
            } else {
                break;
            }
        }
        stack.push((lcp[i - 1], len));
        x += lcp[i - 1] * len;
        ans[sa.sa()[i]] += x;
    }

    let mut stack: Vec<(usize, usize)> = vec![];
    let mut x = 0;
    for i in (0..n - 1).rev() {
        let mut len = 1;
        while stack.len() > 0 {
            if stack.last().unwrap().0 >= lcp[i] {
                let (v, l) = stack.pop().unwrap();
                len += l;
                x -= v * l;
            } else {
                break;
            }
        }
        stack.push((lcp[i], len));
        x += lcp[i] * len;
        ans[sa.sa()[i]] += x;
    }

    for i in 0..n {
        ans[i] += n - i;
    }

    println!("{}", ans.iter().join("\n"));
}
