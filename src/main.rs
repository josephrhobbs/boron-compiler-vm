// crate::main

mod vm;
mod util;

use vm::{memory, interpreter};
use util::config;

fn main() {
    let configuration: config::Config = config::configure();
    let mut virtual_machine = memory::initialize_vm();
    virtual_machine.load_program(configuration.program);
    
    interpreter::interpret(&mut virtual_machine);
}