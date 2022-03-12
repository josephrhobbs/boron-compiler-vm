mod vm;
mod util;

use vm::memory;
use util::config;

fn main() {
    let configuration: config::Config = config::configure();
    println!("{:?}", configuration.program);

    let mut virtual_machine = memory::initialize_vm();
    virtual_machine.load_program(configuration.program);
}