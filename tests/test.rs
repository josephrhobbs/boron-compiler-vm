use boron::vm::{memory, interpreter};
use boron::util::config;
use boron::assm::assembler;

#[test]
fn bin00() {
    // TODO: Fix this so it can read test00 files
    let configuration: config::BinConfig = config::binconfigure();
    let mut virtual_machine = memory::initialize();
    virtual_machine.load_program(configuration.program);

    interpreter::interpret(&mut virtual_machine);

    dbg!(&virtual_machine.registers);
}

#[test]
fn assm00() {
    // TODO: Fix this so it can read test00 files
    let configuration: config::TxtConfig = config::txtconfigure();
    let program: Vec<&str> = configuration.program.lines().collect();

    let bytecode: Vec<u8> = assembler::assemble(program);
    dbg!(&bytecode);

    let output_filename: String = configuration.name;
    dbg!(&output_filename);
}