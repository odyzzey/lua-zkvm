use methods::{METHOD_NAME_ELF, METHOD_NAME_ID};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Executor, ExecutorEnv,
};

fn main() {

    let x: u64 = 7;
    let y: u64 = 8;

    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&x).unwrap())
        .add_input(&to_vec(&y).unwrap())
        .build();

    let mut exec = Executor::from_elf(env, METHOD_NAME_ELF).unwrap();
    let session = exec.run().unwrap();
    let z: u64 = from_slice(session.journal.as_slice()).unwrap();

    println!("{x} * {y} = {z}");
    // Prints: 7 * 8 = 56 
    
    // proving completes but receipt verification fails
    // session.prove().unwrap();

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    //receipt.verify(METHOD_NAME_ID).unwrap();
}
