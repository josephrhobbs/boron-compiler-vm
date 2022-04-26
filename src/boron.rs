// crate::boronc

use std::fs::File;
use std::io::Write;
use std::env;

use colored::Colorize;

use boron::util::{config, cli};
use boron::util::cli::{CLArgs, CLCommand};
use boron::assm::assembler;
use boron::vm::{memory, interpreter};
use boron::util::error::{BoronError, throw};

const VERSION: &str = "0.5.2";
const RELEASE: &str = "alpha";
const AUTHOR: &str = "Developed by Joseph Hobbs (https://github.com/hobbsbros/)";

// MAIN FUNCTION - Program entry point
fn main() {
    // For formatting purposes
    println!("");

    let args: CLArgs = cli::args();

    // Get command type from CLArgs
    let command_type = match args.command {
        Some(ref c) => c,
        None => {
            // ERROR - no subcommand specified
            // Show help menu
            help();
            return;
        }
    };

    // User has specified a file to compile from Boron source code to bytecode
    if command_type == &CLCommand::Compile {
        compile(args);
        return;
    }

    if command_type == &CLCommand::Assemble {
        assemble(args);
        return;
    }

    if command_type == &CLCommand::Help {
        help();
        return;
    }

    if command_type == &CLCommand::Exec {
        exec(args);
        return;
    }

    if command_type == &CLCommand::Version {
        version_info();
        return;
    }
}

fn compile(args: CLArgs) {
    let filename: &str = match args.filename {
        Some(ref s) => s,
        None => {
            throw(BoronError::CommandLineError("No filename specified.".to_string()));
            return;
        }
    };

    // Get the filename from command-line argument
    let _configuration: config::TxtConfig = config::txtconfigure(filename);

    // TODO: Tokenize input

    // TODO: Parse into syntax trees

    // TODO: Evaluate each tree and emit Boron assembly

    // TODO: Compile assembly to bytecode

    // TODO: Save bytecode to file

    throw(BoronError::UnimplementedError);
}

fn assemble(args: CLArgs) {
    let filename: &str = match args.filename {
        Some(ref s) => s,
        None => {
            throw(BoronError::CommandLineError("No filename specified.".to_string()));
            return;
        }
    };

    // Get the filename from command-line argument
    let configuration: config::TxtConfig = config::txtconfigure(filename);
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
    println!("{} {} bytes {} {}{}\n", "Wrote".green(), bytecode.len(), "to file:".green(), output_filename, ".bex");
}

fn exec(args: CLArgs) {
    let filename: &str = match args.filename {
        Some(ref s) => s,
        None => {
            throw(BoronError::CommandLineError("No filename specified.".to_string()));
            return;
        }
    };

    let configuration: config::BinConfig = config::binconfigure(&filename);
    let mut virtual_machine = memory::initialize();
    virtual_machine.load_program(configuration.program);

    interpreter::interpret(&mut virtual_machine);

    // For debugging purposes only
    println!("Virtual machine registers:");
    println!("{:#?}\n", virtual_machine.registers);
}

fn help() {
    println!(
        "{}\n\n{}\n\n{}{}\n",

        "The Boron Language Compiler".bold().yellow(),

        "USAGE:\n\tboron [SUBCOMMAND] [FILENAME]",

        "AVAILABLE SUBCOMMANDS:\
        \n\tcompile: compile Boron source code to Boron executable bytecode\
        \n\tassemble: assemble Boron assembly language to Boron executable bytecode\
        \n\texec: execute Boron bytecode on the Boron virtual machine\
        \n\tversion: get Boron version information\
        \n\thelp: display this help menu",

        "Running `boron` with no subcommands will display this help menu."
    );
}

fn version_info() {
    println!("{}\n{} {} {}\n{}\n", "The Boron Language Compiler".bold().yellow(), "Version".bold(), VERSION.bold(), RELEASE.bold(), AUTHOR.bold());
}