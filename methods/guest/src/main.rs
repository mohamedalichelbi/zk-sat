#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


use risc0_zkvm::guest::env;
use splr::Certificate;

extern crate alloc;

use alloc::vec::Vec;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let sat_problem: Vec<Vec<i32>> = env::read();
    // solve
    let result: &str = match Certificate::try_from(sat_problem) {
        Ok(Certificate::SAT(_ans)) => "SATISFIABLE",
        Ok(Certificate::UNSAT) => "UNSATISFIABLE",
        Err(e) => "UNKNOWN",
    };

    env::commit(&result);
}
