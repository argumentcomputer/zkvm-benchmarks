use methods::{GUEST_RISC0_FIB_ELF, GUEST_RISC0_FIB_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::Serialize;
use std::fmt::Debug;
use std::str::FromStr;
use std::time::Instant;

#[derive(Serialize)]
struct Stats {
    program: &'static str,
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
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // setup
    let n = env_or("FIB_ARG", 100000u64);
    let it = Instant::now();
    let env = ExecutorEnv::builder().write(&n).unwrap().build().unwrap();
    let prover = default_prover();
    let setup_secs = it.elapsed().as_secs_f32();

    // proof
    let it = Instant::now();
    let prove_info = prover.prove(env, GUEST_RISC0_FIB_ELF).unwrap();
    let prove_secs = it.elapsed().as_secs_f32();

    let receipt = prove_info.receipt;
    let (proof_n, res): (u64, u64) = receipt.journal.decode().unwrap();
    assert_eq!(n, proof_n);

    eprintln!("fib({n}) = {res}");

    // verify
    let it = Instant::now();
    receipt.verify(GUEST_RISC0_FIB_ID).unwrap();
    let verify_secs = it.elapsed().as_secs_f32();

    let stats = Stats {
        program: "fib-risc0",
        n,
        prove_secs,
        verify_secs,
        setup_secs,
    };

    println!("{}", serde_json::to_string(&stats).unwrap());
}
