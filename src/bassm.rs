// crate::bassm

use boron::util::config;
use boron::assm::assembler;

fn main() {
    let configuration: config::TxtConfig = config::txtconfigure();
    let program: Vec<&str> = configuration.program.lines().collect();

    let bytecode: Vec<u8> = assembler::assemble(program);
    let output_filename: String = configuration.name;
}