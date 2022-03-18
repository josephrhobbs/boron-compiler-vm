// crate::bassm

mod util;
mod assm;

use util::config;
use assm::assembler;

fn main() {
    let configuration: config::TxtConfig = config::txtconfigure();
    let program: Vec<&str> = configuration.program.lines().collect();

    let bytecode: Vec<u8> = assembler::assemble(program);
}