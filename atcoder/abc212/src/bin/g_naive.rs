use proconio::input;

fn main() {
    input! {
        p: u64,
    };

    let mut cnt = 0;
    for x in 0..p {
        for y in 0..p {
            if (1..=2 * p).any(|i| pow_mod(x, i, p) == y) {
                cnt += 1;
            }
        }
    }
    println!("{cnt}");
}

const fn pow_mod(mut base: u64, mut exp: u64, p: u64) -> u64 {
    let mut x = 1;
    base %= p;
    while exp > 0 {
        if exp & 1 == 1 {
            x = x * base % p;
        }
        base = base * base % p;
        exp >>= 1;
    }
    x
}
