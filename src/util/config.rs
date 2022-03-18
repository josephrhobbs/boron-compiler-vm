// crate::util::config
// Configures files on start-up

use std::env;
use std::io::Read;
use std::fs;

pub struct BinConfig {
    pub program: Vec<u8>,
}

pub struct TxtConfig {
    pub program: String,
}

pub fn binconfigure() -> BinConfig {
    let args: Vec<String> = env::args().collect();
    let name: String = args[1].clone();

    // file must be mutable here as it is being read later in the program
    // We also pass &name rather than name to prevent changing the parent scope of name
    let mut file = fs::File::open(&name).expect("Could not find filename");
    let metadata = fs::metadata(&name).expect("Unable to read metadata");

    let mut buffer = vec![0u8; metadata.len() as usize];
    // We pass &mut buffer to prevent changing the scope of buffer while still permitting modification
    file.read(&mut buffer).expect("Buffer overflow");

    BinConfig {program: buffer}
}

pub fn txtconfigure() -> TxtConfig {
    let args: Vec<String> = env::args().collect();
    let name: String = args[1].clone();

    // file must be mutable here as it is being read later in the program
    // We also pass &name rather than name to prevent changing the parent scope of name
    let mut file = fs::File::open(&name).expect("Could not find filename");

    let mut buffer = String::new();
    // We pass &mut buffer to prevent changing the scope of buffer while still permitting modification
    file.read_to_string(&mut buffer).expect("Buffer overflow");

    TxtConfig {program: buffer}
}