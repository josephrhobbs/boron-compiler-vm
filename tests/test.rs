// boron tester file

// NB: "cargo test" executes from the root directory of the corresponding crate

use boron::vm::{memory, interpreter};
use boron::util::config;
use boron::assm::assembler;

#[test]
fn bin00() {
    let configuration: config::BinConfig = config::binconfigure_from_filename("tests/test00/test00.bex");
    let mut virtual_machine = memory::initialize();
    virtual_machine.load_program(configuration.program);

    interpreter::interpret(&mut virtual_machine);
}

#[test]
fn assm00() {
    let configuration: config::TxtConfig = config::txtconfigure_from_filename("tests/test00/test00.bsm");
    let program: Vec<&str> = configuration.program.lines().collect();

    let bytecode: Vec<u8> = assembler::assemble(program);

    let output_filename: String = configuration.name;
}