// crate::util::cli

use std::env;

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
    pub filename: String,
    pub command: Option<CLCommand>,
    pub flags: Vec<CLFlag>,
}

// Get and parse command-line arguments
pub fn args() -> CLArgs {
    let arguments: Vec<String> = env::args().collect();
    let mut filename: String = String::new();
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
            if filename != String::new() {
                filename = String::from(i);
            } else {
                todo!("Throw an error because filename has already been defined");
            }
        }
    }

    CLArgs {filename: filename, command: command_option, flags: flags}
}