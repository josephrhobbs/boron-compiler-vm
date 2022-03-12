// crate::vm::memory
// Memory management for the Boron Virtual Machine

// BVM overview:
//     2^28 = 268435456 bytes (268 MB) of virtual RAM
//     2^26 = 67108864 bytes (67 MB) of program storage
//
//     15 64-bit general-purpose registers
//     1 32-bit program counter

// Memory addressing:
//     0x00 00 00 00 to 0x03 FF FF FF -> program storage
//     0x04 00 00 00 to 0x13 FF FF FF -> virtual random access memory

const RAM_SIZE: usize = 268435356;
const MEM_SIZE: usize = 67108864;
const NUM_REGISTERS: usize = 15;

pub struct VirtualMachine {
    pub ram: Vec<u8>,
    pub program_storage: Vec<u8>,
    pub registers: Vec<u64>,
    pub pc: u32,
}

pub fn initialize_vm() -> VirtualMachine {
    VirtualMachine {ram: vec![0u8; RAM_SIZE], program_storage: vec![0u8; MEM_SIZE], registers: vec![0u64; NUM_REGISTERS], pc: 0u32}
}

impl VirtualMachine {
    // Loads programs into virtual memory
    pub fn load_program(&mut self, program: Vec<u8>) {
        let mut new_program = program;
        while new_program.len() < MEM_SIZE {
            new_program.push(0u8);
        }
        self.program_storage = new_program;
    }
}