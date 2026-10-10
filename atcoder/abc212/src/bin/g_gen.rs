use cplib::arithmetic::prime::is_prime;
use rand_gen::*;

fn main() {
    let seed: u64 = std::env::args().nth(1).unwrap().parse().unwrap();
    let mut rng = Rng::new(seed);

    let mut p = 1;
    while !is_prime(p) {
        p = rng.range(1, 500) as u32;
    }
    println!("{p}");
}
