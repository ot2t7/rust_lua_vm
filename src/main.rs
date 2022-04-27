use std::fs;
use std::env;

mod error;
mod vm;

fn main() {
    // Get the arguments passed into the executable
    let args: Vec<String> = env::args().collect();

    // First argument is our filename for the bytecode
    let file_packed = fs::read(args[1].clone());
    match file_packed {
        Ok(_) => {},
        Err(_) => error::err(error::ErrorKind::CouldntReadFile)
    }

    let file = file_packed.unwrap();

    vm::run_vm(file);
}
