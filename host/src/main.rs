// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`
use methods::{CERTIFY_ELF, CERTIFY_ID};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Executor, ExecutorEnv, ExecutorEnvBuilder
};
use std::time::{Instant, Duration};
use rand::Rng;
use dimacs::{ parse_dimacs, Instance, Sign };


fn main() {
    let max_vars: u64 = ((std::i32::MAX as i64) - (std::i32::MIN as i64)).try_into().unwrap();
    let prb_name = "p1";
    
    let prb_file = format!("./splr-in/{}.cnf", prb_name);
    let file_content = std::fs::read_to_string(prb_file)
        .expect(&format!("failed to read file content for problem {}", prb_name));
    let prb_instance = parse_dimacs(&file_content)
        .expect(&format!("failed to parse dimacs for problem {}", prb_name));

    // extract clauses
    let (num_vars, clauses) = match prb_instance {
        Instance::Cnf { num_vars, clauses } => (num_vars, clauses),
        _ => unimplemented!("dimacs parsed file is not CNF"),
    };

    println!("Problem {} has {:?} clauses", prb_name, &clauses.len());

    // convert clauses from i64 to i32 (the latter is used by the splr solver)
    if num_vars > max_vars { unimplemented!("Problem {} has too many variables", prb_name); }
    let splr_prb: Vec<Vec<i32>> = clauses.iter()
        .map(|clause| {
            let splr_literal: Vec<i32> = clause.lits().iter()
                .map(|literal| {
                    let var_name: i32 = literal.var().0.try_into().unwrap();
                    let final_var = match literal.sign() {
                        Sign::Pos => var_name,
                        Sign::Neg => -var_name,
                    };
                    final_var
                })
                .collect();
            splr_literal
        })
        .collect();
    
    // First, we construct an executor environment
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&splr_prb).unwrap())
        .build();

    // Next, we make an executor, loading the (renamed) ELF binary.
    let mut exec = Executor::from_elf(env, CERTIFY_ELF).unwrap();

    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    let start_time_prover = Instant::now();

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();

    println!("Prover duration {:?}", start_time_prover.elapsed());

    println!("Receipt size {:.2} (KB)", (to_vec(&receipt).unwrap().len() / 1024));

    let start_time_verifier = Instant::now();

    // verify your receipt
    receipt.verify(CERTIFY_ID).unwrap();

    println!("Verifier duration {:?}", start_time_verifier.elapsed());
}
