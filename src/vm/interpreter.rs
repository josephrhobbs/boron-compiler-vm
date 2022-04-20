// crate::vm::interpreter
// Interpreter for Boron Virtual Machine bytecode

use std::io;
use std::io::Write;

use console::Term;

use super::memory;

// Executes instructions stored in the Virtual Machine, starting at address 0x00000000
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
            let register = vm.next();
            let value: u64 = vm.get_u64();

            vm.to_register(value, register);
        }

        // 0x02 ADD
        else if byte == 2u8 {
            let register = vm.next();
            let inc = vm.get_u64();
            let value = vm.from_register(register);

            vm.to_register(value + inc, register);
        }

        // 0x02 ADD
        else if byte == 2u8 {
            let register = vm.next();
            let inc = vm.get_u64();
            let value = vm.from_register(register);

            vm.to_register(value + inc, register);
        }

        // 0x03 SUB
        else if byte == 3u8 {
            let register = vm.next();
            let inc = vm.get_u64();
            let value = vm.from_register(register);

            vm.to_register(value - inc, register);
        }

        // 0x11 LD
        else if byte == 17u8 {
            let register = vm.next();
            let pointer = vm.get_u64() as u32;

            let value = vm.get(pointer) as u64;
            vm.to_register(value, register);
        }

        // 0x12 STO
        else if byte == 17u8 {
            let register = vm.next();
            let pointer = vm.get_u64() as u32;
            
            let value = vm.from_register(register) as u8;
            vm.store(value, pointer);
        }

        // 0x21 LDR
        else if byte == 33u8 {
            let destination_register = vm.next();
            let pointer_register = vm.next();

            let pointer = vm.from_register(pointer_register) as u32;

            let value = vm.get(pointer) as u64;
            vm.to_register(value, destination_register);
        }

        // 0x22 STR
        else if byte == 34u8 {
            let source_register = vm.next();
            let pointer_register = vm.next();

            let pointer = vm.from_register(pointer_register) as u32;

            let value = vm.from_register(source_register) as u8;
            vm.store(value, pointer);
        }

        // 0x31 LSL
        else if byte == 49u8 {
            let register = vm.next();
            let places = vm.next();

            let value = vm.from_register(register);
            vm.to_register(value << places, register);
        }

        // 0x32 LSR
        else if byte == 50u8 {
            let register = vm.next();
            let places = vm.next();

            let value = vm.from_register(register);
            vm.to_register(value >> places, register);
        }

        // 0x41 JMP
        else if byte == 65u8 {
            let pointer: u32 = vm.get_u64() as u32;
            vm.set_pc(pointer);
        }

        // 0x42 JLT
        else if byte == 66u8 {
            let pointer: u32 = vm.get_u64() as u32;
            let r1: u8 = vm.next();
            let r2: u8 = vm.next();

            let v1: u64 = vm.from_register(r1);
            let v2: u64 = vm.from_register(r2);

            if v1 < v2 {
                vm.set_pc(pointer);
            }
        }

        // 0x43 JLE
        else if byte == 67u8 {
            let pointer: u32 = vm.get_u64() as u32;
            let r1: u8 = vm.next();
            let r2: u8 = vm.next();

            let v1: u64 = vm.from_register(r1);
            let v2: u64 = vm.from_register(r2);

            if v1 <= v2 {
                vm.set_pc(pointer);
            }
        }

        // 0x44 JGT
        else if byte == 68u8 {
            let pointer: u32 = vm.get_u64() as u32;
            let r1: u8 = vm.next();
            let r2: u8 = vm.next();

            let v1: u64 = vm.from_register(r1);
            let v2: u64 = vm.from_register(r2);

            if v1 > v2 {
                vm.set_pc(pointer);
            }
        }

        // 0x45 JGE
        else if byte == 69u8 {
            let pointer: u32 = vm.get_u64() as u32;
            let r1: u8 = vm.next();
            let r2: u8 = vm.next();

            let v1: u64 = vm.from_register(r1);
            let v2: u64 = vm.from_register(r2);

            if v1 >= v2 {
                vm.set_pc(pointer);
            }
        }

        // 0x46 JEQ
        else if byte == 70u8 {
            let pointer: u32 = vm.get_u64() as u32;
            let r1: u8 = vm.next();
            let r2: u8 = vm.next();

            let v1: u64 = vm.from_register(r1);
            let v2: u64 = vm.from_register(r2);

            if v1 == v2 {
                vm.set_pc(pointer);
            }
        }

        // 0x47 JNE
        else if byte == 71u8 {
            let pointer: u32 = vm.get_u64() as u32;
            let r1: u8 = vm.next();
            let r2: u8 = vm.next();

            let v1: u64 = vm.from_register(r1);
            let v2: u64 = vm.from_register(r2);

            if v1 != v2 {
                vm.set_pc(pointer);
            }
        }

        // 0x48 JPR
        else if byte == 72u8 {
            let register: u8 = vm.next();
            let value: u64 = vm.from_register(register);

            vm.set_pc(value as u32);
        }

        // 0x51 PUSH
        else if byte == 81u8 {
            let register: u8 = vm.next();
            let value: u64 = vm.from_register(register);
            vm.push(value);
        }

        // 0x52 POP
        else if byte == 82u8 {
            let register: u8 = vm.next();
            let value: u64 = vm.pop();
            vm.to_register(value, register);
        }

        // 0x61 CALL
        else if byte == 97u8 {
            // We need to copy the register values to avoid issues
            for value in vm.registers.clone().iter() {
                vm.push(*value);
            }

            // Set the PC to the next instruction *without consuming the next instruction*
            vm.push((vm.pc + 8) as u64);

            let pointer: u32 = vm.get_u64() as u32;
            vm.set_pc(pointer);
        }

        // 0x62 RETURN
        else if byte == 98u8 {
            // Get the return value, pop the return address from the top of the stack, and then push the return value to the stack
            let return_register = vm.next();
            let return_value = vm.from_register(return_register);
            let return_pointer = vm.pop();

            // Recover registers
            let mut new_registers: Vec<u64> = Vec::new();
            for _ in 1..=vm.registers.len() {
                new_registers.push(vm.pop());
            }
            new_registers.reverse();
            vm.registers = new_registers;

            vm.push(return_value);
            vm.set_pc(return_pointer as u32);
        }

        // 0xA1 TX
        else if byte == 161u8 {
            let pointer = vm.get_u64() as u32;
            let byte: char = vm.get(pointer) as char;

            print!("{}", byte);
            io::stdout().flush().unwrap();
        }

        // 0xA2 RX
        else if byte == 162u8 {
            let pointer = vm.get_u64() as u32;
            let terminal = Term::stdout();
            let byte: u8 = terminal.read_char().unwrap() as u8;

            vm.store(byte, pointer);
        }

        // 0xFF HLT
        else if byte == 255u8 {
            break;
        }
    }
}