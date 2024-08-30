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

pub fn main() {
    // setup
    let it = Instant::now();
    let (prove, verify) = guest::build_lcs();
    let setup_secs = it.elapsed().as_secs_f32();

    let args : (String, String) = (env_or("LCS_ARG1", "When in the Course of human events, it becomes necessary for one people to dissolve the political bands which have connected them with another".into()),
       env_or("LCS_ARG2", "There must be some kind of way outta here Said the joker to the thief. There's too much confusion. I can't get no relief.".into()));
    // proof
    let it = Instant::now();
    let (output, proof) = prove(args.clone());
    let prove_secs = it.elapsed().as_secs_f32();

    eprintln!("lcs({:?}, {:?}) = {output}", args.0, args.1);

    // verify
    let it = Instant::now();
    let is_valid = verify(proof);
    let verify_secs = it.elapsed().as_secs_f32();
    assert!(is_valid);

    let stats = Stats {
        program: "lcs-jolt",
        args,
        prove_secs,
        verify_secs,
        setup_secs,
    };

    println!("{}", serde_json::to_string(&stats).unwrap());
}
