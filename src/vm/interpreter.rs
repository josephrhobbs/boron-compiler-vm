// crate::vm::interpreter
// Interpreter for Boron Virtual Machine bytecode

use super::memory;

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
        // 0xFF HLT
        else if byte == 255u8 {
            break;
        }
    }
}