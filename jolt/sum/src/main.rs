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

pub fn main() {
    // setup
    let it = Instant::now();
    let (prove, verify) = guest::build_sum();
    let setup_secs = it.elapsed().as_secs_f32();

    let max_num = env_or("SUM_ARG", 100000u64);
    let nums: Vec<u64> = (0..max_num).collect::<Vec<_>>();
    // proof
    let it = Instant::now();
    let (output, proof) = prove(nums);
    let prove_secs = it.elapsed().as_secs_f32();

    eprintln!("sum(0..{max_num}) = {output}");

    // verify
    let it = Instant::now();
    let is_valid = verify(proof);
    let verify_secs = it.elapsed().as_secs_f32();
    assert!(is_valid);

    let stats = Stats {
        program: "sum-jolt",
        n: max_num,
        prove_secs,
        verify_secs,
        setup_secs,
    };

    println!("{}", serde_json::to_string(&stats).unwrap());
}
