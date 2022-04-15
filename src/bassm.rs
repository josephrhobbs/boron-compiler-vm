// crate::bassm

use std::fs::File;
use std::io::Write;
use std::env;

use boron::util::config;
use boron::assm::assembler;

fn main() {
    // Get the filename from command-line argument
    let configuration: config::TxtConfig = config::txtconfigure();
    // Read the file and prepare it for assembly
    let program: Vec<&str> = configuration.program.lines().collect();

    // Assemble the program
    let bytecode: Vec<u8> = assembler::assemble(program);
    let output_filename: String = configuration.name;

    // Write the output to a file
    let path = env::current_dir().unwrap();
    let mut bex_file = File::create(path.join(output_filename.clone() + ".bex")).unwrap();
    bex_file.write(&bytecode[..]).unwrap();

    // Report back to user
    println!("Wrote {} bytes to {}", bytecode.len(), output_filename + ".bex");
}