// crate::bassm

use std::fs::File;
use std::io::Write;
use std::env;

use boron::util::config;
use boron::assm::assembler;

fn main() {
    let configuration: config::TxtConfig = config::txtconfigure();
    let program: Vec<&str> = configuration.program.lines().collect();

    let bytecode: Vec<u8> = assembler::assemble(program);
    let output_filename: String = configuration.name;

    let path = env::current_dir().unwrap();
    let mut bex_file = File::create(path.join(output_filename.clone() + ".bex")).unwrap();
    bex_file.write(&bytecode[..]).unwrap();

    println!("Wrote {} bytes to {}", bytecode.len(), output_filename + ".bex");
}