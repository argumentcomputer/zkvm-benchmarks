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
    let (prove, verify) = guest::build_fastfib();
    let setup_secs = it.elapsed().as_secs_f32();

    let n = env_or("FASTFIB_ARG", u64::MAX - 1);
    // proof
    let it = Instant::now();
    let (output, proof) = prove(n);
    let prove_secs = it.elapsed().as_secs_f32();

    eprintln!("fastfib({n}) = {output}");

    // verify
    let it = Instant::now();
    let is_valid = verify(proof);
    let verify_secs = it.elapsed().as_secs_f32();
    assert!(is_valid);

    let stats = Stats {
        program: "fastfib-jolt",
        n,
        prove_secs,
        verify_secs,
        setup_secs,
    };

    println!("{}", serde_json::to_string(&stats).unwrap());
}
