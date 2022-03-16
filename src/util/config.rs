// crate::util::config
// Configures files on start-up

use std::env;
use std::io::Read;
use std::fs;

pub struct Config {
    pub program: Vec<u8>,
}

pub fn configure() -> Config {
    let args: Vec<String> = env::args().collect();
    let name: String = args[1].clone();

    // file must be mutable here as it is being read later in the program
    // We also pass &name rather than name to prevent changing the parent scope of name
    let mut file = fs::File::open(&name).expect("Could not find filename");
    let metadata = fs::metadata(&name).expect("Unable to read metadata");

    let mut buffer = vec![0u8; metadata.len() as usize];
    // We pass &mut buffer to prevent changing the scope of buffer while still permitting modification
    file.read(&mut buffer).expect("Buffer overflow");

    Config {program: buffer}
}

pub fn configure_from_filename(filename: String) -> Config {
    let mut file = fs::File::open(&filename).expect("Could not find filename");
    let metadata = fs::metadata(&filename).expect("Unable to read metadata");

    let mut buffer = vec![0u8; metadata.len() as usize];
    file.read(&mut buffer).expect("Buffer overflow");

    Config {program: buffer}
}