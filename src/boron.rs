// crate::boronc

use std::fs::File;
use std::io::Write;
use std::env;

use colored::Colorize;

use boron::util::{config, cli};
use boron::util::cli::{CLArgs, CLCommand};
use boron::assm::assembler;
use boron::vm::{memory, interpreter};

// MAIN FUNCTION - Program entry point
fn main() {
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
}

fn compile(args: CLArgs) {
    println!("\n{}\n", "ERROR: Boron source code compilation support has not yet been added.".bold().red());
}

fn assemble(args: CLArgs) {
    let filename: String = args.filename;

    // Get the filename from command-line argument
    let configuration: config::TxtConfig = config::txtconfigure(&filename);
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
    println!("\n{} {} {} {}{}\n", "Wrote".green(), bytecode.len(), "to file".green(), output_filename.green(), ".bex".green());
}

fn exec(args: CLArgs) {
    let filename: String = args.filename;

    let configuration: config::BinConfig = config::binconfigure(&filename);
    let mut virtual_machine = memory::initialize();
    virtual_machine.load_program(configuration.program);

    interpreter::interpret(&mut virtual_machine);

    // For debugging purposes only
    println!("\nVirtual machine registers:");
    println!("{:#?}\n", virtual_machine.registers);
}

fn help() {
    println!("\n{}\n", "Help menu... coming soon!".bold().yellow());
}

fn no_command() {
    println!("\n{}", "No sub-command specified.".bold().red());
    println!("{}\n", "Sub-commands supported are `compile` and `assemble`.".bold().red())
}