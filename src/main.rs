mod error;
mod vm;

use std::fs;
use std::env;
use error::*;

fn main() {
    // Get the arguments passed into the executable
    let args: Vec<String> = env::args().collect();

    // First argument is our filename for the bytecode
    let file_packed = fs::read(args[1].clone());
    match file_packed {
        Ok(_) => {},
        Err(_) => ErrorKind::couldnt_read_file()
    }

    let file = file_packed.unwrap();

    vm::from_bytecode(file);
}
