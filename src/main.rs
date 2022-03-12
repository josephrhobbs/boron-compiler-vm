mod vm;
mod util;

use vm::memory;
use util::config;
use std::fs;

fn main() {
    let configuration: config::Config = config::configure();
    println!("{:?}", configuration.program);

    let mut virtual_machine = memory::initialize_vm();
    virtual_machine.load_program(vec![1u8; 100]);
}