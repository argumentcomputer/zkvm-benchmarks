use risc0_zkvm::guest::env;

fn main() {
    let input: Vec<u64> = env::read();

    let output: u64 = input.iter().sum();

    env::commit(&(input, output));
}
