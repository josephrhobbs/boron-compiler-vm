// crate::vm::memory
// Memory management for the Boron Virtual Machine

// BVM overview:
//     2^28 = 268435456 bytes (268 MB) of virtual RAM
//     2^26 = 67108864 bytes (67 MB) of program storage
//
//     15 64-bit general-purpose registers
//     1 32-bit program counter

// Memory map:
//     0x00 00 00 00 to 0x03 FF FF FF -> program storage
//     0x04 00 00 00 to 0x04 00 FF FF -> output buffer
//     0x04 01 00 00 to 0x04 01 FF FF -> input buffer
//     0x04 02 00 00 to 0x13 FF FF FF -> virtual random access memory

// Register map:
//     0x0 to 0xE -> general-purpose registers
//     0xF        -> program counter

const MEM_SIZE: usize = 335544319;
const NUM_REGISTERS: usize = 15;

pub struct VirtualMachine {
    pub memory: Vec<u8>,
    pub registers: Vec<u64>,
    pub pc: u32,
}

pub fn initialize_vm() -> VirtualMachine {
    VirtualMachine {memory: vec![0u8; MEM_SIZE], registers: vec![0u64; NUM_REGISTERS], pc: 0u32}
}

impl VirtualMachine {
    // Loads programs from Config into virtual memory
    pub fn load_program(&mut self, program: Vec<u8>) {
        let new_program = &program;
        self.memory.splice(0..program.len(), new_program.iter().cloned());
    }
    // Gets a value from memory at a given pointer
    pub fn get(&mut self, pointer: u32) -> u8 {
        self.memory[pointer as usize]
    }
    // Loads a value into memory at a given pointer
    pub fn store(&mut self, value: u8, pointer: u32) {
        self.memory[pointer as usize] = value;
    }
    // Gets a value from memory, with the pointer given by a value in a given register
    pub fn get_register(&mut self, register: u8) -> u8 {
        let pointer: usize = self.registers[register as usize] as usize;
        self.memory[pointer as usize]
    }
    // Loads a value into memory, with the pointer given by a value in a given register
    pub fn store_register(&mut self, value: u8, register: u8) {
        let pointer: usize = self.registers[register as usize] as usize;
        self.memory[pointer as usize] = value;
    }
    // Store a value in a given register
    pub fn to_register(&mut self, value: u64, register: u8) {
        self.registers[register as usize] = value;
    }
    // Get a value from a register
    pub fn from_register(&mut self, register: u8) -> u64 {
        self.registers[register as usize]
    }
    // Get the value at the current program counter
    pub fn next(&mut self) -> u8 {
        self.inc(1);
        self.memory[self.pc as usize]
    }
    // Increment the program counter by a specific number
    pub fn inc(&mut self, increment: u32) {
        self.pc += increment;
    }
    // Gets a slice of 8 bytes and converts it into one 64-bit value
    pub fn get_u64(&mut self) -> u64 {
        let start = self.pc;
        let slice = self.memory[(start as usize)..((start+8u32) as usize)].to_vec();
        self.inc(8);
        let mut result: u64 = 0;
        for value in slice {
            result += value as u64;
            result = result << 1;
        }
        result
    }
}