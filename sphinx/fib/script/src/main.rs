use serde::Serialize;
use sphinx_core::utils::SphinxCoreOpts;
use sphinx_sdk::{utils, ProverClient, SphinxStdin};
use std::fmt::Debug;
use std::str::FromStr;
use std::time::Instant;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

#[derive(Serialize)]
struct Stats {
    program: &'static str,
    shard_size: usize,
    reconstruct_commitments: bool,
    shard_batch_size: usize,
    shard_chunking_multiplier: usize,
    n: u64,
    prove_secs: f32,
    verify_secs: f32,
    setup_secs: f32,
}

fn env_or<T: FromStr>(var: &str, def: T) -> T
where
    <T as FromStr>::Err: Debug,
{
    std::env::var(var)
        .map(|s| {
            s.parse::<T>()
                .unwrap_or_else(|_| panic!("Could not parse {}", var))
        })
        .unwrap_or(def)
}

fn main() {
    utils::setup_logger();

    // setup
    let it = Instant::now();
    let n = env_or("FIB_ARG", 100000u64);
    let mut stdin = SphinxStdin::new();
    stdin.write(&n);
    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let setup_secs = it.elapsed().as_secs_f32();

    // proof
    let it = Instant::now();
    let mut proof = client.prove(&pk, stdin).run().unwrap();
    let prove_secs = it.elapsed().as_secs_f32();

    let proof_n = proof.public_values.read::<u64>();
    let res = proof.public_values.read::<u64>();
    assert_eq!(n, proof_n);

    eprintln!("fib({n}) = {res}");

    // verify
    let it = Instant::now();
    client.verify(&proof, &vk).expect("verification failed");
    let verify_secs = it.elapsed().as_secs_f32();

    let opts = SphinxCoreOpts::default();
    let stats = Stats {
        program: "fib-sphinx",
        shard_size: opts.shard_size,
        reconstruct_commitments: opts.reconstruct_commitments,
        shard_batch_size: opts.shard_batch_size,
        shard_chunking_multiplier: opts.shard_chunking_multiplier,
        n,
        prove_secs,
        verify_secs,
        setup_secs,
    };

    println!("{}", serde_json::to_string(&stats).unwrap());
}
