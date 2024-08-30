#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let n: u64 = sp1_zkvm::io::read::<u64>();

    sp1_zkvm::io::commit(&n);

    // Compute the n'th fibonacci number, using normal Rust code.
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 0..n {
        // Naturally overflow at 64 bits
        let c: u64 = a.wrapping_add(b);
        a = b;
        b = c;
    }

    sp1_zkvm::io::commit(&a);
    sp1_zkvm::io::commit(&b);
}
