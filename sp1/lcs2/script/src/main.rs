use serde::Serialize;
use sp1_sdk::{utils, ProverClient, SP1Stdin};
use sp1_stark::SP1CoreOpts;
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
    utils::setup_logger();

    // setup
    let it = Instant::now();
    let args : (String, String) = (env_or("LCS2_ARG1", "When in the Course of human events, it becomes necessary for one people to dissolve the political bands which have connected them with another".into()),
       env_or("LCS2_ARG2", "There must be some kind of way outta here Said the joker to the thief. There's too much confusion. I can't get no relief.".into()));
    let mut stdin = SP1Stdin::new();
    stdin.write(&args);
    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let setup_secs = it.elapsed().as_secs_f32();

    // proof
    let it = Instant::now();
    let mut proof = client.prove(&pk, stdin).run().unwrap();
    let prove_secs = it.elapsed().as_secs_f32();

    let proof_args = proof.public_values.read::<(String, String)>();
    let res = proof.public_values.read::<String>();
    assert_eq!(proof_args, args);

    eprintln!("lcs2({:?}, {:?}) = {res}", args.0, args.1);

    // verify
    let it = Instant::now();
    client.verify(&proof, &vk).expect("verification failed");
    let verify_secs = it.elapsed().as_secs_f32();

    let opts = SP1CoreOpts::default();
    let stats = Stats {
        program: "lcs2-sp1",
        shard_size: opts.shard_size,
        reconstruct_commitments: opts.reconstruct_commitments,
        shard_batch_size: opts.shard_batch_size,
        args: proof_args,
        prove_secs,
        verify_secs,
        setup_secs,
    };

    println!("{}", serde_json::to_string(&stats).unwrap());
}
