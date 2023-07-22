use clap::{Arg, ArgAction, Command};
use methods::{VALIDATE_POV_ELF, VALIDATE_POV_ID};
use risc0_zkvm::{
    default_executor_from_elf,
    serde::{from_slice, to_vec},
    ExecutorEnv,
};

fn read_bin_from_file(path: &String) -> Vec<u8> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path).expect("read file failed");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    contents
}

fn main() {
    // Parse command line
    let matches = Command::new("validate")
        .arg(
            Arg::new("param")
                .short('p')
                .action(ArgAction::Append)
                .required(true),
        )
        .arg(Arg::new("debug").short('d').action(ArgAction::SetTrue))
        .get_matches();

    let log_level = if matches.get_flag("debug") {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    env_logger::builder().filter_level(log_level).init();

    let param_paths: Vec<&String> = matches
        .get_many::<String>("param")
        .expect("we should use `-p` to select pov params file!")
        .collect();

    log::info!("params: {:?}", param_paths);

    let params = param_paths
        .into_iter()
        .map(|path| read_bin_from_file(path))
        .collect::<Vec<_>>();

    // First, we construct an executor environment
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&params).expect("to_vec failed"))
        .build()
        .unwrap();

    // Next, we make an executor, loading the (renamed) ELF binary.
    let mut exec = default_executor_from_elf(env, VALIDATE_POV_ELF).unwrap();

    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();

    // TODO: Implement code for transmitting or serializing the receipt for
    // other parties to verify here

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(VALIDATE_POV_ID).unwrap();
}
