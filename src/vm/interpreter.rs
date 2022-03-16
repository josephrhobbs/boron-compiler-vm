// crate::vm::interpreter
// Interpreter for Boron Virtual Machine bytecode

use super::memory;
use std::io;
use std::io::Write;

// Executes instructions stored in the Vrtual Machine, starting at address 0x00000000
pub fn interpret(vm: &mut memory::VirtualMachine) {
    // We iterate forever until we receive the halt operation
    loop {
        let byte = vm.next();

        // 0x00 NOP
        if byte == 0u8 {
            // Literally do nothing
            // This command only exists as an explicit way to do specify the zero operation
        }
        // 0x01 STO
        else if byte == 1u8 {
            let destination_register = vm.next();
            let value: u64 = vm.get_u64();

            vm.to_register(value, destination_register);
        }
        // 0x02 ADD
        else if byte == 2u8 {
            let register = vm.next();
        }
        // 0x41 JMP
        else if byte == 65u8 {
            let pointer: u32 = vm.get_u32();
            vm.set_pc(pointer);
        }
        // 0xA1 TX
        else if byte == 161u8 {
            print!("{}", vm.next() as char);
            io::stdout().flush().unwrap();
        }
        // 0xA2 RX
        else if byte == 162u8 {

        }
        // 0xFF HLT
        else if byte == 255u8 {
            break;
        }
    }
}