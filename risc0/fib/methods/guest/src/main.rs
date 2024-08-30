use risc0_zkvm::guest::env;

fn main() {
    let n: u64 = env::read();

    // Compute the n'th fibonacci number, using normal Rust code.
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 0..n {
        // Naturally overflow at 64 bits
        let c: u64 = a.wrapping_add(b);
        a = b;
        b = c;
    }

    env::commit(&(n, a));
}
