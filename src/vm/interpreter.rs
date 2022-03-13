// crate::vm::interpreter
// Interpreter for Boron Virtual Machine bytecode

use super::memory;

// Gets a slice of 8 bytes and converts it into one 64-bit value
fn convert_to_u64(vm: &memory::VirtualMachine, start: u32) -> u64 {
    let slice = &(*vm).memory[(start as usize)..((start+8u32) as usize)];

    let mut result: u64 = 0;
    for value in slice {
        result += *value as u64;
        result = result << 1;
    }
    result
}

// Executes instructions stored in the Vrtual Machine, starting at address 0x00000000
pub fn interpret(vm: &mut memory::VirtualMachine) {
    // We iterate forever until we receive the halt operation
    loop {
        let byte: u8 = (*vm).memory[(*vm).pc as usize];

        (*vm).pc += 1;

        // 0x00 NOP
        if byte == 0u8 {
            // Literally do nothing
            // This command only exists as an explicit way to do specify the empty operation
        }
        // 0x01 STO
        else if byte == 1u8 {
            let destination_register: usize = (*vm).memory[(*vm).pc as usize] as usize;
            (*vm).pc += 1;

            // NOTE TO SELF: I use .to_vec() here to convert a slice into a Vec
            let value: u64 = convert_to_u64(&vm, (*vm).pc);
            (*vm).pc += 8;

            (*vm).registers[destination_register] = value;
        }
        // 0xFF HLT
        else if byte == 255u8 {
            break;
        }
    }
}