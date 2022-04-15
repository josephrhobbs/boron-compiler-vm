// crate::bvm

use boron::vm::{memory, interpreter};
use boron::util::config;

fn main() {
    let configuration: config::BinConfig = config::binconfigure();
    let mut virtual_machine = memory::initialize();
    virtual_machine.load_program(configuration.program);

    interpreter::interpret(&mut virtual_machine);

    // For debugging purposes only
    println!("Virtual machine registers:");
    println!("{:#?}", virtual_machine.registers);
}