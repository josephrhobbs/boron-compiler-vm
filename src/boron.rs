// crate::boronc

use std::fs::File;
use std::io::Write;
use std::env;

use boron::util::config;
use boron::assm::assembler;

fn assemble() {
    // Get the filename from command-line argument
    let configuration: config::TxtConfig = config::txtconfigure();
    // Read the file and prepare it for assembly
    let program: Vec<&str> = configuration.program.lines().collect();

    // Assemble the program
    let bytecode: Vec<u8> = assembler::assemble(program);
    let output_filename: String = configuration.name;

    // Write the output to a file
    // TODO: Implement more robust error handling
    let path = env::current_dir().unwrap();
    let mut bex_file = File::create(path.join(output_filename.clone() + ".bex")).unwrap();
    bex_file.write(&bytecode[..]).unwrap();

    // Report back to user
    println!("Wrote {} bytes to {}", bytecode.len(), output_filename + ".bex");
}

fn compile() {
    println!("Boron source code compilation support has not yet been added.")
}

fn help() {
    println!("Help menu... coming soon!");
}

fn main() {
    let cli_args: Vec<String> = env::args().collect();

    // User has not specified any input, most likely wants help
    if cli_args.len() == 1 {
        help();
        return;
    }

    let command_type = cli_args[1].clone();

    // User has specified a file to compile from Boron source code to bytecode
    if command_type == String::from("compile") {
        compile();
    }

    else if command_type == String::from("assemble") {
        assemble();
    }
}