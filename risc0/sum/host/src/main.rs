use methods::{GUEST_RISC0_SUM_ELF, GUEST_RISC0_SUM_ID};
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
    let max_num = env_or("SUM_ARG", 100000u64);
    let nums: Vec<u64> = (0..max_num).collect::<Vec<_>>();
    let it = Instant::now();
    let env = ExecutorEnv::builder()
        .write(&nums)
        .unwrap()
        .build()
        .unwrap();
    let prover = default_prover();
    let setup_secs = it.elapsed().as_secs_f32();

    // proof
    let it = Instant::now();
    let prove_info = prover.prove(env, GUEST_RISC0_SUM_ELF).unwrap();
    let prove_secs = it.elapsed().as_secs_f32();

    let receipt = prove_info.receipt;
    let (proof_nums, res): (Vec<u64>, u64) = receipt.journal.decode().unwrap();
    assert_eq!(proof_nums.len(), max_num as usize);

    eprintln!("sum(0..{max_num}) = {res}");

    // verify
    let it = Instant::now();
    receipt.verify(GUEST_RISC0_SUM_ID).unwrap();
    let verify_secs = it.elapsed().as_secs_f32();

    let stats = Stats {
        program: "sum-risc0",
        n: max_num,
        prove_secs,
        verify_secs,
        setup_secs,
    };

    println!("{}", serde_json::to_string(&stats).unwrap());
}
