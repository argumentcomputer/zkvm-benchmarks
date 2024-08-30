use p3_baby_bear::BabyBear;
use p3_field::AbstractField;
use serde::Serialize;
use sphinx_core::{
    stark::{LocalProver, StarkGenericConfig, StarkMachine},
    utils::{BabyBearPoseidon2, SphinxCoreOpts},
};
use std::fmt::Debug;
use std::str::FromStr;
use std::time::Instant;

use loam::{
    lair::{
        chipset::Chipset,
        execute::{QueryRecord, Shard},
        func_chip::FuncChip,
        lair_chip::{build_chip_vector, LairMachineProgram},
        toplevel::Toplevel,
        List,
    },
    lurk::{
        chipset::LurkChip,
        eval::build_lurk_toplevel,
        zstore::{lurk_zstore, ZPtr, ZStore},
    },
};

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

fn build_lurk_expr(arg: u64) -> String {
    format!(
        "(letrec ((fib
          (lambda (n)
            (if (<= n 1) n
              (+ (fib (- n 1)) (fib (- (- n 1) 1)))))))
  (fib {arg}))"
    )
}

#[allow(clippy::type_complexity)]
fn setup<H: Chipset<BabyBear>>(
    arg: u64,
    toplevel: &Toplevel<BabyBear, H>,
) -> (
    List<BabyBear>,
    FuncChip<'_, BabyBear, H>,
    QueryRecord<BabyBear>,
    ZStore<BabyBear, LurkChip>,
) {
    let code = build_lurk_expr(arg);
    let mut zstore = lurk_zstore();
    let ZPtr { tag, digest } = zstore.read(&code).unwrap();

    let mut record = QueryRecord::new(toplevel);
    record.inject_inv_queries("hash_32_8", toplevel, &zstore.hashes4);

    let mut full_input = [BabyBear::zero(); 24];
    full_input[0] = tag.to_field();
    full_input[8..16].copy_from_slice(&digest);

    let args: List<_> = full_input.into();
    let lurk_main = FuncChip::from_name("lurk_main", toplevel);

    (args, lurk_main, record, zstore)
}

fn main() {
    // setup
    let it = Instant::now();
    let arg = env_or("FIB_ARG", 100000u64);
    let (toplevel, _) = build_lurk_toplevel();
    let (args, lurk_main, mut record, mut zstore) = setup(arg, &toplevel);
    let config = BabyBearPoseidon2::new();
    let opts = SphinxCoreOpts::default();
    let setup_secs = it.elapsed().as_secs_f32();

    // proof
    let it = Instant::now();
    let res = toplevel
        .execute(lurk_main.func(), &args, &mut record, None)
        .unwrap();
    let machine = StarkMachine::new(
        config,
        build_chip_vector(&lurk_main),
        record.expect_public_values().len(),
    );
    let (pk, vk) = machine.setup(&LairMachineProgram);
    let mut challenger_p = machine.config().challenger();
    let shard = Shard::new(&record);
    let proof = machine.prove::<LocalProver<_, _>>(&pk, shard, &mut challenger_p, opts);
    let prove_secs = it.elapsed().as_secs_f32();

    let res = ZPtr::from_flat_data(&res);
    zstore.memoize_dag(
        res.tag,
        &res.digest,
        record.get_inv_queries("hash_24_8", &toplevel),
        record.get_inv_queries("hash_32_8", &toplevel),
        record.get_inv_queries("hash_48_8", &toplevel),
    );
    eprintln!("fib({arg}) = {}", zstore.fmt(&res));

    // verify
    let it = Instant::now();
    let mut challenger_v = machine.config().challenger();
    machine
        .verify(&vk, &proof, &mut challenger_v)
        .expect("verification failed");
    let verify_secs = it.elapsed().as_secs_f32();

    let stats = Stats {
        program: "fib-loam",
        shard_size: opts.shard_size,
        reconstruct_commitments: opts.reconstruct_commitments,
        shard_batch_size: opts.shard_batch_size,
        shard_chunking_multiplier: opts.shard_chunking_multiplier,
        n: arg,
        prove_secs,
        verify_secs,
        setup_secs,
    };

    println!("{}", serde_json::to_string(&stats).unwrap());
}
