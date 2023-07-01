// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`
use methods::{MULTIPLY_ELF, MULTIPLY_ID};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Executor, ExecutorEnv, ExecutorEnvBuilder
};
use std::time::{Instant, Duration};
use rand::Rng;

fn main() {
    let start_time = Instant::now();

    let mut rng = rand::thread_rng();

    let a: u8 = rng.gen();
    let b: u8 = rng.gen();
    
    // First, we construct an executor environment
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&a).unwrap())
        .add_input(&to_vec(&b).unwrap())
        .build();

    // Next, we make an executor, loading the (renamed) ELF binary.
    let mut exec = Executor::from_elf(env, MULTIPLY_ELF).unwrap();

    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    println!("Session creation duration {:?}", start_time.elapsed());
    let start_time_prover = Instant::now();

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();

    println!("Prover duration {:?}", start_time_prover.elapsed());

    println!("Receipt size {:.2} (KB)", (to_vec(&receipt).unwrap().len() / 1024));

    let start_time_verifier = Instant::now();

    // verify your receipt
    receipt.verify(MULTIPLY_ID).unwrap();

    println!("Verifier duration {:?}", start_time_verifier.elapsed());
    println!("Full duration {:?}", start_time.elapsed());
}
