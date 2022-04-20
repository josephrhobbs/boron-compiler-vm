// crate::util::config
// Configures files on start-up

use std::io::Read;
use std::fs;

use super::error::{BoronError, throw};

pub struct BinConfig {
    pub program: Vec<u8>,
}

pub struct TxtConfig {
    pub program: String,
    pub name: String,
}

pub fn binconfigure(filename: &str) -> BinConfig {
    let name: String = String::from(filename);

    // file must be mutable here as it is being read later in the program
    // We also pass &name rather than name to prevent changing the parent scope of name
    let mut file = match fs::File::open(&name) {
        Ok(f) => f,
        _ => {
            throw(BoronError::FileNotFoundError);
            return BinConfig {program: Vec::new()};
        },
    };
    let metadata = fs::metadata(&name).expect("Unable to read metadata");

    let mut buffer = vec![0u8; metadata.len() as usize];
    // We pass &mut buffer to prevent changing the scope of buffer while still permitting modification
    match file.read(&mut buffer) {
        Ok(_) => {},
        _ => {
            throw(BoronError::BufferOverflowError);
            return BinConfig {program: Vec::new()};
        }
    };

    BinConfig {program: buffer}
}

pub fn txtconfigure(filename: &str) -> TxtConfig {
    let name: String = String::from(filename);

    // Remove the extension from the name (so that we can put a .bex extension on it after assembly)
    let split_by_dot: Vec<&str> = name.split(".").collect();
    let len_split: usize = split_by_dot.len();
    let name_ext_rmvd: String = String::from(split_by_dot[..len_split-1].join("."));

    // file must be mutable here as it is being read later in the program
    // We also pass &name rather than name to prevent changing the parent scope of name
    let mut file = match fs::File::open(&name) {
        Ok(f) => f,
        _ => {
            throw(BoronError::FileNotFoundError);
            return TxtConfig {program: String::new(), name: name_ext_rmvd};
        },
    };

    let mut buffer = String::new();
    // We pass &mut buffer to prevent changing the scope of buffer while still permitting modification
    match file.read_to_string(&mut buffer) {
        Ok(_) => {},
        _ => {
            throw(BoronError::BufferOverflowError);
            return TxtConfig {program: String::new(), name: name_ext_rmvd};
        }
    };

    TxtConfig {program: buffer, name: name_ext_rmvd}
}