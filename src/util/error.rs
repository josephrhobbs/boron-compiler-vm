// crate::util::error

use colored::Colorize;

pub enum BoronError {
    CommandLineError (String),
    FileNotFoundError,
    BufferOverflowError,
    UnimplementedError,
    BoronSyntaxError (String),
}

pub fn throw(error: BoronError) {
    let prefix = String::from("ERROR:");
    let suffix = match error {
        BoronError::CommandLineError(s) => s + " Exiting.",
        BoronError::FileNotFoundError => "Could not find the file specified.".to_string(),
        BoronError::BufferOverflowError => "Input buffer overflowed while trying to read input file.".to_string(),
        BoronError::UnimplementedError => "This feature has not been implemented.".to_string(),
        BoronError::BoronSyntaxError(s) => "Invalid syntax. ".to_owned() + &s,
    };
    println!("{} {}\n", prefix.bold().red(), suffix.bold().white());
}