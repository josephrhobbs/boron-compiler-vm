// crate::util::config
// Configures files on start-up

use std::env;
use std::fs;

pub struct Config {
    pub program: Vec<u8>,
}

pub fn configure() -> Config {
    let args: Vec<String> = env::args().collect();
    let name: String = args[0].clone();
    let p = fs::read(name).unwrap();
    Config {program: p}
}