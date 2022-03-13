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

const PGRM_SIZE: usize = 67108863;
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
    // Loads programs into virtual memory
    pub fn load_program(&mut self, program: Vec<u8>) {
        let mut new_program = &program;
        self.memory.splice(0..program.len(), new_program.iter().cloned());
    }
    pub fn get(&mut self, pointer: u32) -> u8 {
        self.memory[pointer as usize]
    }
    pub fn load(&mut self, value: u8, pointer: u32) {
        self.memory[pointer as usize] = value;
    }
    pub fn get_register(&mut self, register: u8) -> u8 {
        let pointer: usize = self.registers[register as usize] as usize;
        self.memory[pointer as usize]
    }
    pub fn load_register(&mut self, value: u8, register: u8) {
        let pointer: usize = self.registers[register as usize] as usize;
        self.memory[pointer as usize] = value;
    }
}