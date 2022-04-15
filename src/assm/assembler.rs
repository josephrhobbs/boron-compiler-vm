// crate::assm::assembler

pub fn tokenize(program: Vec<&str>) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current = String::new();

    'outer: for line in program {
        // line: &str
        for c in line.chars() {
            // c: char
            if c == '#' {
                // # indicates a comment, so do nothing
                continue 'outer;
            } else if c == ' ' {
                // Spaces separate tokens
                let next_token: String = current.clone();
                tokens.push(next_token);
                current = String::new();
            } else {
                // Push to the next token
                current.push(c);
            }
        }
        let next_token: String = current.clone();
        if !next_token.is_empty() {
            tokens.push(next_token);
        }
        current = String::new();
    }

    tokens
}

// Reverses the order of bytes for use during assembly
fn reverse_bytes(value: u64) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut v: u64 = value;
    for _ in 0..8 {
        result.push((v % 256) as u8);
        v = v >> 8;
    }
    result.into_iter().rev().collect()
}

pub fn assemble(program: Vec<&str>) -> Vec<u8> {
    let mut bytecode: Vec<u8> = Vec::new();

    let tokens: Vec<String> = tokenize(program);

    // Convert tokens to bytecode
    for token in tokens {
        let t: &str = &String::from(token).to_lowercase();

        // NB: Use .collect::<T>() to collect into a specified type
        let first_char: char = t.chars().collect::<Vec<char>>()[0];

        // 0x00 NOP
        if t == "nop" {
            bytecode.push(0);
        }

        // 0x01 SET
        if t == "set" {
            bytecode.push(1);
        }

        // 0x02 ADD
        else if t == "add" {
            bytecode.push(2);
        }

        // 0x03 SUB
        else if t == "sub" {
            bytecode.push(3);
        }

        // 0x11 LD
        else if t == "ld" {
            bytecode.push(17);
        }

        // 0x12 STO
        else if t == "sto" {
            bytecode.push(18);
        }

        // 0x21 LDR
        else if t == "ldr" {
            bytecode.push(33);
        }

        // 0x22 STR
        else if t == "str" {
            bytecode.push(34);
        }

        // 0x31 LSL
        else if t == "lsl" {
            bytecode.push(49);
        }

        // 0x32 LSR
        else if t == "lsr" {
            bytecode.push(50);
        }

        // 0x41 JMP
        else if t == "jmp" {
            bytecode.push(65);
        }

        // 0x42 JLT
        else if t == "jlt" {
            bytecode.push(66);
        }

        // 0x43 JLE
        else if t == "jle" {
            bytecode.push(67);
        }

        // 0x44 JGT
        else if t == "jgt" {
            bytecode.push(68);
        }

        // 0x45 JGE
        else if t == "jge" {
            bytecode.push(69);
        }

        // 0x46 JEQ
        else if t == "jeq" {
            bytecode.push(70);
        }

        // 0x47 JNE
        else if t == "jne" {
            bytecode.push(71);
        }

        // 0x48 JPR
        else if t == "jpr" {
            bytecode.push(72);
        }

        // 0x51 PUSH
        else if t == "push" {
            bytecode.push(81);
        }

        // 0x52 POP
        else if t == "pop" {
            bytecode.push(82);
        }

        // 0xA1 TX
        else if t == "tx" {
            bytecode.push(161);
        }

        // 0xA2 RX
        else if t == "rx" {
            bytecode.push(162);
        }

        // 0xFF HLT
        else if t == "halt" {
            bytecode.push(255);
        }

        // Token represents a register
        else if first_char == 'r' {
            let register_str: &str = &(t[1..].to_string());
            
            let register: u8 = register_str.parse::<u8>().unwrap();
            bytecode.push(register);
        }

        // Token represents a literal u64, attempt to parse it as an integer
        else if let Ok(value) = t.parse::<u64>() {
            bytecode.append(&mut reverse_bytes(value));
        }
    }

    // Allows for implicit halting at the end of a program
    // The programmer may also explicitly specify a halt in a specific subroutine
    bytecode.push(255);

    bytecode
}