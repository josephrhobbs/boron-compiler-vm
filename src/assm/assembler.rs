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

    // dbg! macro takes ownership of variables
    dbg!(&tokens);

    tokens
}

pub fn assemble(program: Vec<&str>) -> Vec<u8> {
    let mut bytecode: Vec<u8> = Vec::new();

    let tokens: Vec<String> = tokenize(program);

    // TODO: Conduct subroutine management

    bytecode = vec![0u8];

    // Convert tokens to bytecode
    for token in tokens {
        // TODO: Is there a .lower() method for String?  If so, implement here
        let t: &str = &token;

        // NB: Use .collect::<T>() to collect into a specified type
        let first_char: char = t.chars().collect::<Vec<char>>()[0];

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

        // 0xA1 TX
        else if t == "tx" {
            bytecode.push(161);
        }

        // 0xA2 RX
        else if t == "rx" {
            bytecode.push(162);
        }

        // 0xFF HLT
        else if t == "hlt" {
            bytecode.push(255);
        }

        // Token represents a register
        else if first_char == 'r' {
            let register_str: &str = &(t[1..].to_string());
            
            // TODO: convert register_str into a u8
            // e.g. &str: "12" -> u8: 12
        }
    }

    // Allows for implicit halting at the end of a program
    // The programmer may also explicitly specify a halt in a specific subroutine
    bytecode.push(255);

    bytecode
}