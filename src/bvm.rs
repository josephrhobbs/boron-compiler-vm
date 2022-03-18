// crate::bvm

mod vm;
mod util;

use vm::{memory, interpreter};
use util::config;

fn main() {
    let configuration: config::BinConfig = config::binconfigure();
    let mut virtual_machine = memory::initialize();
    virtual_machine.load_program(configuration.program);

    interpreter::interpret(&mut virtual_machine);
}