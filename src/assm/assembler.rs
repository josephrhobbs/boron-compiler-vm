// crate::assm::assembler

pub fn assemble(program: Vec<&str>) -> Vec<u8> {
    let mut bytecode: Vec<u8> = Vec::new();
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

    println!("{:#?}", tokens);

    bytecode = vec![0u8];
    // TODO: Implement conversion between tokens and output bytecode

    bytecode
}