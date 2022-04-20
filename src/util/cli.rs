// crate::util::cli

use std::env;

use super::error::{throw, BoronError};

#[derive(PartialEq)]
pub enum CLCommand {
    Compile,
    Assemble,
    Exec,
    Help,
}

pub enum CLFlag {

}

pub struct CLArgs {
    pub filename: Option<String>,
    pub command: Option<CLCommand>,
    pub flags: Vec<CLFlag>,
}

// Get and parse command-line arguments
pub fn args() -> CLArgs {
    let arguments: Vec<String> = env::args().collect();
    let mut filename: String = String::new();
    let mut filename_option = None;
    let mut command_option: Option<CLCommand> = None;

    let flags: Vec<CLFlag> = Vec::new();

    // Ignore `boron` command
    for i in arguments[1..].iter() {
        // Coerce the element to a &str for easier processing
        let item: &str = &i;

        // Check Boron subcommands
        if item == "compile" && command_option == None {
            command_option = Some(CLCommand::Compile);
        }

        else if item == "assemble" && command_option == None {
            command_option = Some(CLCommand::Assemble);
        }

        else if item == "exec" && command_option == None {
            command_option = Some(CLCommand::Exec);
        }

        else if item == "help" && command_option == None {
            command_option = Some(CLCommand::Help);
        }

        // TODO: Check Boron compiler CLI flags

        // Assume this is a filename
        else {
            if filename == String::new() {
                filename = String::from(i);
            } else {
                throw(BoronError::CommandLineError("Only one filename may be passed as an argument.".to_string()));
                return CLArgs {filename: None, command: None, flags: Vec::new()}
            }
        }
    }

    // Assign filename_option a Some() value if a filename was provided
    if filename != String::new() {
        filename_option = Some(filename);
    }

    CLArgs {filename: filename_option, command: command_option, flags: flags}
}