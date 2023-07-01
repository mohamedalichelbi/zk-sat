#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let a: u8 = env::read();
    let b: u8 = env::read();

    let c = a * b;

    env::commit(&c);
}
