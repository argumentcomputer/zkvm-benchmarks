use methods::{GUEST_RISC0_LCS2_ELF, GUEST_RISC0_LCS2_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::Serialize;
use std::fmt::Debug;
use std::str::FromStr;
use std::time::Instant;

#[derive(Serialize)]
struct Stats {
    program: &'static str,
    args: (String, String),
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
    let args : (String, String) = (env_or("LCS2_ARG1", "When in the Course of human events, it becomes necessary for one people to dissolve the political bands which have connected them with another".into()),
       env_or("LCS2_ARG2", "There must be some kind of way outta here Said the joker to the thief. There's too much confusion. I can't get no relief.".into()));
    let it = Instant::now();
    let env = ExecutorEnv::builder()
        .write(&args)
        .unwrap()
        .build()
        .unwrap();
    let prover = default_prover();
    let setup_secs = it.elapsed().as_secs_f32();

    // proof
    let it = Instant::now();
    let prove_info = prover.prove(env, GUEST_RISC0_LCS2_ELF).unwrap();
    let prove_secs = it.elapsed().as_secs_f32();

    let receipt = prove_info.receipt;
    let (proof_args, res): ((String, String), String) = receipt.journal.decode().unwrap();
    assert_eq!(args, proof_args);

    eprintln!("lcs2({:?}, {:?}) = {res}", args.0, args.1);

    // verify
    let it = Instant::now();
    receipt.verify(GUEST_RISC0_LCS2_ID).unwrap();
    let verify_secs = it.elapsed().as_secs_f32();

    let stats = Stats {
        program: "lcs2-risc0",
        args,
        prove_secs,
        verify_secs,
        setup_secs,
    };

    println!("{}", serde_json::to_string(&stats).unwrap());
}
