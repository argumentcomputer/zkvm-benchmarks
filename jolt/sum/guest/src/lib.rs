// #![cfg_attr(feature = "guest", no_std)]
#![no_main]

#[jolt::provable(max_input_size = 100000001)]
fn sum(input: Vec<u64>) -> u64 {
    input.iter().sum()
}
