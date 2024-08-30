#![cfg_attr(feature = "guest", no_std)]
#![no_main]

#[jolt::provable]
fn fib(n: u64) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut sum: u64;
    for _ in 1..n {
        sum = a.wrapping_add(b);
        a = b;
        b = sum;
    }

    b
}
